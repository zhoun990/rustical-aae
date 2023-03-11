use std::str::FromStr;
use sqlx::SqlitePool;
use anyhow::Result;

pub(crate) async fn init_db() -> Result<SqlitePool> {//, Box<dyn std::error::Error>
    // このmain関数はasync fnではないので、asyncな関数を呼ぶのにblock_on関数を使う
    // use tauri::async_runtime::block_on;

    // データベースのファイルパス等を設定する
    const DOCUMENTS_DIR: &str = "Documents";
    const APP_DIR: &str = "Webbel";
    const DATABASE_DIR: &str = "AAE";
    const DATABASE_FILE: &str = "db.sqlite";
    // ユーザのホームディレクトリ直下にデータベースのディレクトリを作成する
    // もし、各OSで標準的に使用されるアプリ専用のデータディレクトリに保存したいなら
    // directoriesクレートの`ProjectDirs::data_dir`メソッドなどを使うとよい
    // https://docs.rs/directories/latest/directories/struct.ProjectDirs.html#method.data_dir
    let home_dir = directories::UserDirs::new()
        .map(|dirs| dirs.home_dir().to_path_buf())
        // ホームディレクトリが取得できないときはカレントディレクトリを使う
        .unwrap_or_else(|| std::env::current_dir().expect("Cannot access the current directory"));
    let documents_dir = home_dir.join(DOCUMENTS_DIR);
    let app_dir = documents_dir.join(APP_DIR);
    let database_dir = app_dir.join(DATABASE_DIR);
    let database_file = database_dir.join(DATABASE_FILE);

    // データベースファイルが存在するかチェックする
    let db_exists = std::fs::metadata(&database_file).is_ok();
    // 存在しないなら、ファイルを格納するためのディレクトリを作成する
    if !db_exists {
        std::fs::create_dir_all(&database_dir);
    }

    let database_dir_str = dunce::canonicalize(&database_dir)
        .unwrap()
        .to_string_lossy()
        .replace('\\', "/");
    let database_url = format!("sqlite://{}/{}", database_dir_str, DATABASE_FILE);
    // SQLiteのコネクションプールを作成する
    let sqlite_pool = create_sqlite_pool(&database_url).await?;
    //  データベースファイルが存在しなかったなら、マイグレーションSQLを実行する
    if !db_exists {
        println!("db:{}, in {:?}", db_exists, database_file);
        sqlx::migrate!("./db").run(&sqlite_pool).await?;
    }
    Ok(sqlite_pool)
}
async fn create_sqlite_pool(database_url: &str) -> Result<SqlitePool> {
    // コネクションの設定
    let connection_options = sqlx::sqlite::SqliteConnectOptions::from_str(database_url)?
        // DBが存在しないなら作成する
        .create_if_missing(true)
        // トランザクション使用時の性能向上のため、WALを使用する
        .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal)
        .synchronous(sqlx::sqlite::SqliteSynchronous::Normal);

    // 上の設定を使ってコネクションプールを作成する
    let sqlite_pool = sqlx::sqlite::SqlitePoolOptions::new()
        .connect_with(connection_options)
        .await?;

    Ok(sqlite_pool)
}