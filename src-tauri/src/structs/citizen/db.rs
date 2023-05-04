use super::{relation::RelationType, *};

#[derive(Clone, Debug, Serialize, Deserialize, sqlx::FromRow)]
pub(crate) struct DBCitizen {
    pub(crate) id: i32,
    pub(crate) name: String,
    pub(crate) born_timestamp: u32,
    pub(crate) death_timestamp: Option<u32>,
    pub(crate) gender: String,
    pub(crate) job: Option<String>,
    pub(crate) staying_city_id: i32,
    pub(crate) home_city_id: i32,
    pub(crate) country_id: Option<i32>,
    pub(crate) level: u16,
    pub(crate) rank: u16,
    pub(crate) exp: u16,
    pub(crate) skill_points: u16,
    pub(crate) money: u16,
}

#[derive(Clone, Debug, Serialize, Deserialize, sqlx::FromRow)]
pub(crate) struct DBRelation {
    pub(crate) id: i32,
    pub(crate) name: String,
    pub(crate) impression: i32,
    pub(crate) relation_type: String,
    pub(crate) last_met_timestamp: u32,
}
impl Citizen {
    pub(super) async fn add_to_db(&mut self) -> sqlx::Result<()> {
        let pool = &getPool();
        let mut tx = pool.begin().await?;
        let id = sqlx::query(
            "INSERT INTO citizens (name, born_timestamp, death_timestamp, gender, job, staying_city_id, home_city_id, country_id, level, rank, exp, skill_points, money)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?);"
        )
        .bind(&self.name)
        .bind(&self.born_timestamp)
        .bind(&self.death_timestamp)
        .bind(match self.gender {
            Gender::Male => "Male",
            Gender::Female => "Female",
        })
        .bind(&self.job)
        .bind(&self.staying_city_id)
        .bind(&self.home_city_id)
        .bind(&self.country_id)
        .bind(&self.level)
        .bind(&self.rank)
        .bind(&self.exp)
        .bind(&self.skill_points)
        .bind(&self.money)
        .execute(&mut tx)
        .await?
        .last_insert_rowid();

        for (target_id, relation) in &self.relations {
            sqlx::query(
                "INSERT INTO relationships (self_id, target_id, impression, relation_type)
            VALUES (?, ?, ?, ?);",
            )
            .bind(id)
            .bind(target_id)
            .bind(relation.impression)
            .bind(match relation.relation_type {
                RelationType::Child => "Child",
                RelationType::Parent => "Parent",
                RelationType::Sibling => "Sibling",
                RelationType::Partner => "Partner",
                RelationType::Acquaintance => "Acquaintance",
                RelationType::Clan => "Clan",
            })
            .execute(&mut tx)
            .await?;
        }
        tx.commit().await?;
        self.id = id as i32;
        Ok(())
    }
    pub(crate) async fn get_from_db() -> sqlx::Result<HashMap<i32, Arc<Mutex<Citizen>>>> {
        let pool = &getPool();

        let raw = sqlx::query_as::<_, DBCitizen>("SELECT * FROM citizens")
            .fetch_all(pool)
            .await?;

        let mut stream = futures::stream::iter(raw).map(|x| async move {
            let mut citizen = Self {
                id: x.id,
                name: x.name.to_string(),
                born_timestamp: x.born_timestamp,
                death_timestamp: x.death_timestamp,
                gender: Gender::from_string(x.gender),
                job: x.job.clone(),
                staying_city_id: x.staying_city_id,
                home_city_id: x.home_city_id,
                country_id: x.country_id,
                relations: Default::default(),
                level: x.level,
                rank: x.rank,
                exp: x.exp,
                skill_points: x.skill_points,
                money: x.money,
            };

            let relations =
                sqlx::query_as::<_, DBRelation>("SELECT * FROM relationships WHERE self_id = ?")
                    .bind(x.id)
                    .fetch_all(pool)
                    .await
                    .unwrap();
            relations.iter().for_each(|y| {
                citizen.relations.insert(
                    y.id,
                    relation::Relation {
                        id: y.id,
                        name: y.name.to_string(),
                        impression: y.impression,
                        relation_type: RelationType::from_string(y.relation_type.to_string()),
                        last_met_timestamp: y.last_met_timestamp,
                    },
                );
            });
            citizen
        });
        let mut map: HashMap<i32, Arc<Mutex<Citizen>>> = Default::default();
        while let Some(citizen) = stream.next().await {
            let citizen = citizen.await;
            map.insert(citizen.id, Arc::new(Mutex::new(citizen)));
        }
        Ok(map)
    }
}
