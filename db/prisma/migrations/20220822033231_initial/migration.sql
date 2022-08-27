-- CreateTable
CREATE TABLE "users" (
    "user_id" BIGINT,
    "nation_id" BIGINT,
    "uuid" UUID
);

-- CreateTable
CREATE TABLE "accounts" (
    "id" SERIAL NOT NULL,
    "name" TEXT,
    "owner_id" BIGINT,
    "alliance_id" INTEGER,
    "resources" resources,
    "war_chest" BOOLEAN,
    "primary" BOOLEAN,
    "deposit_code" TEXT
);

-- CreateTable
CREATE TABLE "alliance_auto_roles" (
    "id" SERIAL NOT NULL,
    "role_id" BIGINT,
    "guild_id" BIGINT,
    "alliance_id" INTEGER,
    "access_level" SMALLINT,
    "condition" TEXT
);

-- CreateTable
CREATE TABLE "alliance_settings" (
    "alliance_id" INTEGER,
    "default_raid_condition" TEXT,
    "default_nuke_condition" TEXT,
    "default_military_condition" TEXT,
    "default_attack_raid_condition" TEXT,
    "default_attack_nuke_condition" TEXT,
    "default_attack_military_condition" TEXT,
    "withdraw_channel_ids" BIGINT[],
    "require_withdraw_approval" BOOLEAN,
    "offshore_id" INTEGER,
    "withdraw_from_offshore" BOOLEAN
);

-- CreateTable
CREATE TABLE "alliances" (
    "id" INTEGER NOT NULL,
    "name" TEXT NOT NULL,
    "acronym" TEXT,
    "score" DECIMAL NOT NULL,
    "color" SMALLINT NOT NULL,
    "date" TIMESTAMPTZ(6) NOT NULL,
    "accepts_members" BOOLEAN NOT NULL,
    "flag" TEXT,
    "forum_link" TEXT,
    "discord_link" TEXT,
    "wiki_link" TEXT,
    "estimated_resources" resources
);

-- CreateTable
CREATE TABLE "alliances_private" (
    "id" INTEGER,
    "resources" resources
);

-- CreateTable
CREATE TABLE "audit_checks" (
    "id" SERIAL NOT NULL,
    "name" TEXT,
    "config_id" INTEGER,
    "condition" TEXT,
    "fail_message_format" TEXT,
    "success_message_format" TEXT,
    "required" BOOLEAN,
    "city" BOOLEAN
);

-- CreateTable
CREATE TABLE "audit_configs" (
    "id" SERIAL NOT NULL,
    "name" TEXT,
    "alliance_id" INTEGER,
    "fail_message_format" TEXT,
    "success_message_format" TEXT
);

-- CreateTable
CREATE TABLE "audit_log_configs" (
    "id" SERIAL NOT NULL,
    "guild_id" BIGINT,
    "channel_id" BIGINT,
    "target_guild_id" BIGINT,
    "target_alliance_id" INTEGER,
    "actions" SMALLINT[]
);

-- CreateTable
CREATE TABLE "audit_logs" (
    "id" SERIAL NOT NULL,
    "config_id" INTEGER,
    "guild_id" BIGINT,
    "channel_id" BIGINT,
    "user_id" BIGINT,
    "alliance_id" INTEGER,
    "action" SMALLINT,
    "data" JSON
);

-- CreateTable
CREATE TABLE "audit_runs" (
    "id" SERIAL NOT NULL,
    "config_id" INTEGER,
    "nation_id" INTEGER,
    "checks" JSON
);

-- CreateTable
CREATE TABLE "bankrecs" (
    "id" INTEGER,
    "date" TIMESTAMPTZ(6),
    "sender_id" INTEGER,
    "sender_type" SMALLINT,
    "receiver_id" INTEGER,
    "receiver_type" SMALLINT,
    "banker_id" INTEGER,
    "note" TEXT,
    "resources" resources,
    "tax_id" INTEGER
);

-- CreateTable
CREATE TABLE "blitz_targets" (
    "id" SERIAL NOT NULL,
    "blitz_id" INTEGER,
    "war_room_id" INTEGER,
    "nation_id" INTEGER,
    "attacker_ids" INTEGER[]
);

-- CreateTable
CREATE TABLE "blitzes" (
    "id" SERIAL NOT NULL,
    "date" TIMESTAMPTZ(6),
    "name" TEXT,
    "message" TEXT,
    "alliance_ids" INTEGER[],
    "planning_alliance_ids" INTEGER[],
    "war_room_config" INTEGER,
    "direct_message" BOOLEAN,
    "in_game_message" BOOLEAN
);

-- CreateTable
CREATE TABLE "bounties" (
    "id" INTEGER,
    "date" TIMESTAMPTZ(6),
    "nation_id" INTEGER,
    "amount" INTEGER,
    "type" SMALLINT
);

-- CreateTable
CREATE TABLE "builds" (
    "id" SERIAL NOT NULL,
    "name" TEXT,
    "owner_id" BIGINT,
    "use_condition" TEXT,
    "infrastructure" DECIMAL,
    "land" DECIMAL,
    "coal_power" INTEGER,
    "oil_power" INTEGER,
    "nuclear_power" INTEGER,
    "wind_power" INTEGER,
    "coal_mines" INTEGER,
    "lead_mines" INTEGER,
    "bauxite_mine" INTEGER,
    "oil_well" INTEGER,
    "uranium_mine" INTEGER,
    "iron_mines" INTEGER,
    "farms" INTEGER,
    "oil_refineries" INTEGER,
    "steel_mills" INTEGER,
    "aluminum_refineries" INTEGER,
    "munitions_factories" INTEGER,
    "police_stations" INTEGER,
    "hospitals" INTEGER,
    "recycling_center" INTEGER,
    "subways" INTEGER,
    "supermarkets" INTEGER,
    "banks" INTEGER,
    "shopping_malls" INTEGER,
    "stadiums" INTEGER,
    "barracks" INTEGER,
    "factories" INTEGER,
    "hangars" INTEGER,
    "drydocks" INTEGER
);

-- CreateTable
CREATE TABLE "cities" (
    "id" INTEGER NOT NULL,
    "nation_id" INTEGER NOT NULL,
    "name" TEXT NOT NULL,
    "date" TIMESTAMPTZ(6) NOT NULL,
    "infrastructure" DECIMAL NOT NULL,
    "land" DECIMAL NOT NULL,
    "powered" BOOLEAN NOT NULL,
    "coal_power" INTEGER NOT NULL,
    "oil_power" INTEGER NOT NULL,
    "nuclear_power" INTEGER NOT NULL,
    "wind_power" INTEGER NOT NULL,
    "coal_mines" INTEGER NOT NULL,
    "lead_mines" INTEGER NOT NULL,
    "bauxite_mine" INTEGER NOT NULL,
    "oil_well" INTEGER NOT NULL,
    "uranium_mine" INTEGER NOT NULL,
    "iron_mines" INTEGER NOT NULL,
    "farms" INTEGER NOT NULL,
    "oil_refineries" INTEGER NOT NULL,
    "steel_mills" INTEGER NOT NULL,
    "aluminum_refineries" INTEGER NOT NULL,
    "munitions_factories" INTEGER NOT NULL,
    "police_stations" INTEGER NOT NULL,
    "hospitals" INTEGER NOT NULL,
    "recycling_center" INTEGER NOT NULL,
    "subways" INTEGER NOT NULL,
    "supermarkets" INTEGER NOT NULL,
    "banks" INTEGER NOT NULL,
    "shopping_malls" INTEGER NOT NULL,
    "stadiums" INTEGER NOT NULL,
    "barracks" INTEGER NOT NULL,
    "factories" INTEGER NOT NULL,
    "hangars" INTEGER NOT NULL,
    "drydocks" INTEGER NOT NULL,
    "nuke_date" TIMESTAMPTZ(6)
);

-- CreateTable
CREATE TABLE "city_auto_roles" (
    "id" SERIAL NOT NULL,
    "role_id" BIGINT,
    "guild_id" BIGINT,
    "min_city" INTEGER,
    "max_city" INTEGER,
    "members_only" BOOLEAN,
    "condition" TEXT
);

-- CreateTable
CREATE TABLE "colors" (
    "date" TIMESTAMPTZ(6),
    "data" TEXT
);

-- CreateTable
CREATE TABLE "conditional_auto_roles" (
    "id" SERIAL NOT NULL,
    "role_id" BIGINT,
    "guild_id" BIGINT,
    "condition" TEXT
);

-- CreateTable
CREATE TABLE "conditions" (
    "id" SERIAL NOT NULL,
    "name" TEXT,
    "owner_id" BIGINT,
    "value" TEXT,
    "use_condition" TEXT
);

-- CreateTable
CREATE TABLE "credentials" (
    "nation_id" INTEGER,
    "api_key" TEXT
);

-- CreateTable
CREATE TABLE "embassies" (
    "id" SERIAL NOT NULL,
    "config_id" INTEGER,
    "guild_id" BIGINT,
    "channel_id" BIGINT,
    "alliance_id" BIGINT,
    "archived" BOOLEAN
);

-- CreateTable
CREATE TABLE "embassy_configs" (
    "id" SERIAL NOT NULL,
    "name" TEXT,
    "category_id" BIGINT,
    "guild_id" BIGINT,
    "message" TEXT,
    "archive_category_id" BIGINT,
    "mentions" INTEGER[],
    "default" BOOLEAN,
    "name_format" TEXT,
    "access_level" SMALLINT
);

-- CreateTable
CREATE TABLE "grants" (
    "id" SERIAL NOT NULL,
    "date" TIMESTAMPTZ(6),
    "status" SMALLINT,
    "recipient" BIGINT,
    "resources" resources,
    "alliance_id" INTEGER,
    "note" TEXT,
    "payoff_type" SMALLINT,
    "deadline" TIMESTAMPTZ(6),
    "expiry" TIMESTAMPTZ(6),
    "paid" resources,
    "payoff_code" TEXT,
    "tax_bracket" INTEGER
);

-- CreateTable
CREATE TABLE "guild_roles" (
    "id" BIGINT,
    "guild_id" BIGINT,
    "permissions" BIGINT
);

-- CreateTable
CREATE TABLE "guild_settings" (
    "guild_id" BIGINT,
    "purpose" SMALLINT,
    "purpose_argument" TEXT,
    "public" BOOLEAN,
    "description" TEXT,
    "welcome_message" TEXT,
    "welcome_channels" BIGINT[],
    "join_role_ids" BIGINT[],
    "verified_role_ids" BIGINT[],
    "member_role_ids" BIGINT[],
    "verified_nickname_format" TEXT,
    "enforce_verified_nickname" BOOLEAN,
    "welcome_mentions" INTEGER[]
);

-- CreateTable
CREATE TABLE "inactive_alerts" (
    "nation_id" INTEGER,
    "last_alert" TIMESTAMPTZ(6)
);

-- CreateTable
CREATE TABLE "interview_answers" (
    "id" SERIAL NOT NULL,
    "question_id" INTEGER,
    "interview_id" INTEGER,
    "answer" TEXT
);

-- CreateTable
CREATE TABLE "interview_configs" (
    "id" SERIAL NOT NULL,
    "name" TEXT,
    "guild_id" BIGINT
);

-- CreateTable
CREATE TABLE "interview_questions" (
    "id" SERIAL NOT NULL,
    "name" TEXT,
    "config_id" INTEGER,
    "question" TEXT,
    "position" INTEGER,
    "answer_type" SMALLINT,
    "choices" TEXT[],
    "min_choices" INTEGER,
    "max_choices" INTEGER
);

-- CreateTable
CREATE TABLE "interviews" (
    "id" SERIAL NOT NULL,
    "config_id" INTEGER,
    "owner_id" BIGINT,
    "ticket_id" INTEGER,
    "require_link" BOOLEAN
);

-- CreateTable
CREATE TABLE "mentions" (
    "id" SERIAL NOT NULL,
    "owner_id" BIGINT,
    "owner_type" SMALLINT,
    "channel_ids" BIGINT[],
    "role_ids" BIGINT[],
    "user_ids" BIGINT[]
);

-- CreateTable
CREATE TABLE "menu_interfaces" (
    "id" SERIAL NOT NULL,
    "menu_id" INTEGER,
    "message_id" BIGINT,
    "channel_id" BIGINT,
    "guild_id" BIGINT
);

-- CreateTable
CREATE TABLE "menu_items" (
    "id" SERIAL NOT NULL,
    "menu_id" INTEGER,
    "type" SMALLINT,
    "style" SMALLINT,
    "label" TEXT,
    "disabled" BOOLEAN,
    "url" TEXT,
    "emoji" BIGINT,
    "action" SMALLINT,
    "action_options" INTEGER[]
);

-- CreateTable
CREATE TABLE "menus" (
    "id" SERIAL NOT NULL,
    "guild_id" BIGINT,
    "name" TEXT,
    "description" TEXT,
    "layout" INTEGER[]
);

-- CreateTable
CREATE TABLE "nations" (
    "id" INTEGER NOT NULL,
    "alliance_id" INTEGER NOT NULL,
    "alliance_position" SMALLINT NOT NULL,
    "name" TEXT NOT NULL,
    "leader" TEXT NOT NULL,
    "continent" SMALLINT NOT NULL,
    "war_policy" SMALLINT NOT NULL,
    "domestic_policy" SMALLINT NOT NULL,
    "color" SMALLINT NOT NULL,
    "num_cities" INTEGER NOT NULL,
    "score" DECIMAL NOT NULL,
    "flag" TEXT NOT NULL,
    "vacation_mode_turns" INTEGER NOT NULL,
    "beige_turns" INTEGER NOT NULL,
    "espionage_available" BOOLEAN NOT NULL,
    "last_active" TIMESTAMPTZ(6) NOT NULL,
    "date" TIMESTAMPTZ(6) NOT NULL,
    "soldiers" INTEGER NOT NULL,
    "tanks" INTEGER NOT NULL,
    "aircraft" INTEGER NOT NULL,
    "ships" INTEGER NOT NULL,
    "missiles" INTEGER NOT NULL,
    "nukes" INTEGER NOT NULL,
    "discord_username" TEXT,
    "turns_since_last_city" INTEGER NOT NULL,
    "turns_since_last_project" INTEGER NOT NULL,
    "projects" INTEGER NOT NULL,
    "wars_won" INTEGER NOT NULL,
    "wars_lost" INTEGER NOT NULL,
    "tax_id" INTEGER NOT NULL,
    "alliance_seniority" INTEGER,
    "estimated_resources" resources
);

-- CreateTable
CREATE TABLE "nations_private" (
    "id" INTEGER,
    "update_tz" DECIMAL,
    "spies" INTEGER,
    "resources" resources
);

-- CreateTable
CREATE TABLE "radiation" (
    "date" TIMESTAMPTZ(6),
    "data" TEXT
);

-- CreateTable
CREATE TABLE "reminders" (
    "id" SERIAL NOT NULL,
    "name" TEXT,
    "message" TEXT,
    "owner_id" BIGINT,
    "mention_ids" INTEGER[],
    "direct_message" BOOLEAN,
    "date" TIMESTAMPTZ(6),
    "interval" interval
);

-- CreateTable
CREATE TABLE "roles" (
    "id" SERIAL NOT NULL,
    "name" TEXT,
    "description" TEXT,
    "alliance_id" INTEGER,
    "rank" INTEGER,
    "permissions" BIGINT,
    "member_ids" BIGINT[],
    "role_ids" BIGINT[],
    "alliance_positions" INTEGER[],
    "privacy_level" INTEGER,
    "access_level" SMALLINT
);

-- CreateTable
CREATE TABLE "rosters" (
    "id" SERIAL NOT NULL,
    "nation_id" INTEGER,
    "alliance_id" INTEGER,
    "join_date" TIMESTAMPTZ(6),
    "time_zone" DECIMAL
);

-- CreateTable
CREATE TABLE "server_submissions" (
    "id" SERIAL NOT NULL,
    "name" TEXT,
    "invite" TEXT,
    "description" TEXT,
    "tags" TEXT[]
);

-- CreateTable
CREATE TABLE "servers" (
    "id" SERIAL NOT NULL,
    "guild_id" BIGINT,
    "name" TEXT,
    "invite" TEXT,
    "description" TEXT,
    "tags" TEXT[]
);

-- CreateTable
CREATE TABLE "subscriptions" (
    "id" SERIAL NOT NULL,
    "guild_id" BIGINT,
    "channel_id" BIGINT,
    "owner_id" BIGINT,
    "event" TEXT,
    "sub_types" TEXT[],
    "condition" TEXT,
    "mentions" INTEGER[]
);

-- CreateTable
CREATE TABLE "tags" (
    "id" SERIAL NOT NULL,
    "name" TEXT,
    "owner_id" BIGINT,
    "text" TEXT,
    "use_condition" TEXT
);

-- CreateTable
CREATE TABLE "target_configs" (
    "id" SERIAL NOT NULL,
    "owner_id" BIGINT,
    "name" TEXT,
    "count" BIGINT,
    "rater" INTEGER,
    "condition" TEXT,
    "use_condition" TEXT,
    "attack" BOOLEAN
);

-- CreateTable
CREATE TABLE "target_raters" (
    "id" SERIAL NOT NULL,
    "cities" TEXT,
    "infrastructure" TEXT,
    "activity" TEXT,
    "soldiers" TEXT,
    "tanks" TEXT,
    "aircraft" TEXT,
    "ships" TEXT,
    "missiles" TEXT,
    "nukes" TEXT,
    "money" TEXT,
    "coal" TEXT,
    "oil" TEXT,
    "uranium" TEXT,
    "iron" TEXT,
    "bauxite" TEXT,
    "lead" TEXT,
    "gasoline" TEXT,
    "munitions" TEXT,
    "steel" TEXT,
    "aluminum" TEXT,
    "food" TEXT
);

-- CreateTable
CREATE TABLE "target_reminders" (
    "id" SERIAL NOT NULL,
    "nation_id" INTEGER,
    "owner_id" BIGINT,
    "mention_ids" INTEGER[],
    "direct_message" BOOLEAN,
    "times" INTEGER[]
);

-- CreateTable
CREATE TABLE "tax_brackets" (
    "id" INTEGER,
    "alliance_id" INTEGER,
    "name" TEXT,
    "date" TIMESTAMPTZ(6),
    "date_modified" TIMESTAMPTZ(6),
    "last_modifier_id" INTEGER,
    "tax_rate" INTEGER,
    "resource_tax_rate" INTEGER
);

-- CreateTable
CREATE TABLE "ticket_configs" (
    "id" SERIAL NOT NULL,
    "name" TEXT,
    "category_id" BIGINT,
    "guild_id" BIGINT,
    "message" TEXT,
    "archive_category_id" BIGINT,
    "mention_ids" INTEGER[],
    "default" BOOLEAN,
    "name_format" TEXT,
    "interview_config_id" INTEGER,
    "close_action" SMALLINT,
    "transcript_channel_id" BIGINT
);

-- CreateTable
CREATE TABLE "tickets" (
    "id" SERIAL NOT NULL,
    "ticket_number" INTEGER,
    "config_id" INTEGER,
    "guild_id" BIGINT,
    "channel_id" BIGINT,
    "owner_id" BIGINT,
    "closed" BOOLEAN
);

-- CreateTable
CREATE TABLE "trades" (
    "id" INTEGER,
    "type" SMALLINT,
    "date" TIMESTAMPTZ(6),
    "sender_id" INTEGER,
    "receiver_id" INTEGER,
    "resource" SMALLINT,
    "amount" INTEGER,
    "action" SMALLINT,
    "price" INTEGER,
    "accepted" BOOLEAN,
    "date_accepted" TIMESTAMPTZ(6),
    "original_trade_id" INTEGER
);

-- CreateTable
CREATE TABLE "transactions" (
    "id" SERIAL NOT NULL,
    "date" TIMESTAMPTZ(6),
    "status" SMALLINT,
    "type" SMALLINT,
    "creator_id" BIGINT,
    "to_id" BIGINT,
    "to_type" SMALLINT,
    "from_id" BIGINT,
    "from_type" SMALLINT,
    "resources" resources,
    "note" TEXT
);

-- CreateTable
CREATE TABLE "treasures" (
    "date" TIMESTAMPTZ(6),
    "data" TEXT
);

-- CreateTable
CREATE TABLE "treaties" (
    "id" INTEGER,
    "date" TIMESTAMPTZ(6),
    "type" SMALLINT,
    "url" TEXT,
    "turns_left" INTEGER,
    "sender_id" INTEGER,
    "receiver_id" INTEGER
);

-- CreateTable
CREATE TABLE "war_attacks" (
    "id" INTEGER,
    "date" TIMESTAMPTZ(6),
    "attacker_id" INTEGER,
    "defender_id" INTEGER,
    "type" SMALLINT,
    "war_id" INTEGER,
    "victor_id" INTEGER,
    "success" INTEGER,
    "attcas1" INTEGER,
    "attcas2" INTEGER,
    "defcas1" INTEGER,
    "defcas2" INTEGER,
    "city_id" INTEGER,
    "infrastructure_destroyed" DECIMAL,
    "improvements_lost" INTEGER,
    "money_stolen" DECIMAL,
    "loot_info" TEXT,
    "resistance_eliminated" INTEGER,
    "city_infrastructure_before" DECIMAL,
    "infrastructure_destroyed_value" DECIMAL,
    "attacker_munitions_used" DECIMAL,
    "defender_munitions_used" DECIMAL,
    "attacker_gasoline_used" DECIMAL,
    "defender_gasoline_used" DECIMAL,
    "aircraft_killed_by_tanks" INTEGER
);

-- CreateTable
CREATE TABLE "war_room_configs" (
    "id" SERIAL NOT NULL,
    "name" TEXT,
    "channel_id" BIGINT,
    "category_ids" BIGINT[],
    "guild_id" BIGINT,
    "message" TEXT,
    "mention_ids" INTEGER[],
    "name_format" TEXT,
    "reuse" BOOLEAN,
    "condition" TEXT,
    "track_wars" BOOLEAN,
    "advise" BOOLEAN
);

-- CreateTable
CREATE TABLE "war_rooms" (
    "id" SERIAL NOT NULL,
    "config_id" INTEGER,
    "guild_id" BIGINT,
    "channel_id" BIGINT,
    "nation_id" INTEGER,
    "war_ids" BIGINT[],
    "archived" BOOLEAN,
    "thread" BOOLEAN
);

-- CreateTable
CREATE TABLE "wars" (
    "id" INTEGER,
    "date" TIMESTAMPTZ(6),
    "reason" TEXT,
    "type" SMALLINT,
    "attacker_id" INTEGER,
    "attacker_alliance_id" INTEGER,
    "defender_id" INTEGER,
    "defender_alliance_id" INTEGER,
    "ground_control" INTEGER,
    "air_superiority" INTEGER,
    "naval_blockade" INTEGER,
    "winner_id" INTEGER,
    "turns_left" INTEGER,
    "attacker_action_points" INTEGER,
    "defender_action_points" INTEGER,
    "attacker_resistance" INTEGER,
    "defender_resistance" INTEGER,
    "attacker_peace" BOOLEAN,
    "defender_peace" BOOLEAN,
    "attacker_fortify" BOOLEAN,
    "defender_fortify" BOOLEAN,
    "attacker_gasoline_used" DECIMAL,
    "defender_gasoline_used" DECIMAL,
    "attacker_munitions_used" DECIMAL,
    "defender_munitions_used" DECIMAL,
    "attacker_aluminum_used" DECIMAL,
    "defender_aluminum_used" DECIMAL,
    "attacker_steel_used" DECIMAL,
    "defender_steel_used" DECIMAL,
    "attacker_infrastructure_destroyed" DECIMAL,
    "defender_infrastructure_destroyed" DECIMAL,
    "attacker_money_looted" DECIMAL,
    "defender_money_looted" DECIMAL,
    "attacker_soldiers_killed" INTEGER,
    "defender_soldiers_killed" INTEGER,
    "attacker_tanks_killed" INTEGER,
    "defender_tanks_killed" INTEGER,
    "attacker_aircraft_killed" INTEGER,
    "defender_aircraft_killed" INTEGER,
    "attacker_ships_killed" INTEGER,
    "defender_ships_killed" INTEGER,
    "attacker_missiles_used" INTEGER,
    "defender_missiles_used" INTEGER,
    "attacker_nukes_used" INTEGER,
    "defender_nukes_used" INTEGER,
    "attacker_infrastructure_destroyed_value" DECIMAL,
    "defender_infrastructure_destroyed_value" DECIMAL
);

-- CreateTable
CREATE TABLE "webhooks" (
    "id" BIGINT,
    "channel_id" BIGINT,
    "guild_id" BIGINT,
    "token" TEXT
);
