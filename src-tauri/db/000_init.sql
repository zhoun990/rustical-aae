-- 人のテーブルを作成する
CREATE TABLE
    IF NOT EXISTS citizens (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL,
        born_timestamp INTEGER NOT NULL,
        death_timestamp INTEGER,
        gender TEXT NOT NULL, --ENUM ('Male', 'Female')
        job TEXT,
        staying_city_id INTEGER NOT NULL,
        home_city_id INTEGER NOT NULL,
        country_id INTEGER,
        level INTEGER NOT NULL,
        rank INTEGER NOT NULL,
        exp INTEGER NOT NULL,
        skill_points INTEGER NOT NULL,
        money INTEGER NOT NULL,
        FOREIGN KEY (staying_city_id) REFERENCES cities (id),
        FOREIGN KEY (home_city_id) REFERENCES cities (id),
        FOREIGN KEY (country_id) REFERENCES countries (id)
    );

CREATE TABLE
    IF NOT EXISTS relationships (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        self_id INTEGER NOT NULL,
        target_id INTEGER NOT NULL,
        impression INTEGER NOT NULL,
        relation_type TEXT NOT NULL, --ENUM ('Child',"Parent","Sibling","Partner","Acquaintance","Clan")
        FOREIGN KEY (self_id) REFERENCES citizen (id),
        FOREIGN KEY (target_id) REFERENCES citizen (id)
    );

-- 街のテーブルを作成する
CREATE TABLE
    IF NOT EXISTS cities (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL,
        position_x INTEGER NOT NULL,
        position_y INTEGER NOT NULL,
        dev_production INTEGER NOT NULL,
        dev_building INTEGER NOT NULL,
        dev_infrastructure INTEGER NOT NULL,
        exp_dev_production INTEGER NOT NULL,
        exp_dev_building INTEGER NOT NULL,
        exp_dev_infrastructure INTEGER NOT NULL,
        control INTEGER NOT NULL,
        environment INTEGER NOT NULL,
        region_id INTEGER NOT NULL,
        country_id INTEGER,
        FOREIGN KEY (region_id) REFERENCES regions (id),
        FOREIGN KEY (country_id) REFERENCES countries (id)
    );

-- 地域のテーブルを作成する
CREATE TABLE
    IF NOT EXISTS regions (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL,
        product TEXT NOT NULL,
        --mapId TEXT NOT NULL,
        position_x INTEGER NOT NULL,
        position_y INTEGER NOT NULL
        -- country_id INTEGER,
        -- FOREIGN KEY (country_id) REFERENCES countries (id)
    );

-- 国のテーブルを作成する
CREATE TABLE
    IF NOT EXISTS countries (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL,
        color_primary TEXT NOT NULL,
        color_secondary TEXT NOT NULL,
        capital_city_id INTEGER NOT NULL,
        FOREIGN KEY (capital_city_id) REFERENCES cities (id)
    );

-- インベントリーのテーブルを作成する
CREATE TABLE
    IF NOT EXISTS items (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL,
        count INTEGER NOT NULL,
        owner_citizen_id INTEGER,
        owner_city_id INTEGER,
        owner_country_id INTEGER,
        FOREIGN KEY (owner_citizen_id) REFERENCES citizens (id),
        FOREIGN KEY (owner_city_id) REFERENCES cities (id),
        FOREIGN KEY (owner_country_id) REFERENCES countries (id),
        CONSTRAINT check_owner CHECK (
            (
                owner_citizen_id IS NOT NULL
                AND owner_city_id IS NULL
                AND owner_country_id IS NULL
            )
            OR (
                owner_citizen_id IS NULL
                AND owner_city_id IS NOT NULL
                AND owner_country_id IS NULL
            )
            OR (
                owner_citizen_id IS NULL
                AND owner_city_id IS NULL
                AND owner_country_id IS NOT NULL
            )
        )
    );