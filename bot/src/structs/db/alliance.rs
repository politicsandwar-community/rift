use model_derive::Model;

#[derive(Clone, Model)]
#[table = "alliances"]
#[primary_key = "alliance_id"]
pub struct Alliance {
    pub alliance_id: Option<i64>,
    pub name: String,
    
  
    pub acronym: String,
    pub score: Float,
    pub color: String,
    pub created_date: DateTimeAuto,
    pub average_score: Float,
    pub accept_members: Boolean,
    pub flag: String,
    pub forum_link: String,
    pub discord_link: String,
    pub wiki_link: String,


    pub nations: Array<Nation>,
}
