-- PWRJRNL in phone keys i think its cute
PRAGMA application_id=7975765;
-- increment with each public schema change
PRAGMA user_version=0;

PRAGMA foreign_keys = ON;

-- small table, mainly to generate ids
create table entries
(
    id       integer not null
        constraint pk
            primary key autoincrement,
    datetime integer not null,
    title    text
);

-- core table, each entry can have multiple pieces
create table piece
(
    id       integer not null
        constraint pk
            primary key autoincrement,
    entry_id integer not null
        constraint fk
            references entries
            on delete cascade,
    -- 0 = text, 1 = mood, 2 = blob, 3 = location, 4 = activities
    type     integer not null
);

-- simple block of text
create table piece_0_text
(
    id      integer
        constraint pk
            primary key
        constraint fk
            references piece
            on delete cascade,

    title   TEXT,
    content TEXT not null
);

-- user rates mood on two scales: pleasantness and energy
create table piece_1_mood
(
    id           integer
        constraint pk
            primary key
        constraint fk
            references piece
            on delete cascade,
    -- integers are fucky, and 30 bits per entry doesnt matter. f64: -1 to 1, just like apple
    pleasantness REAL not null,
    energy       REAL
);

-- large unparsable file: image, audio, video
create table piece_2_blob
(
    id   integer
        constraint pk
            primary key
        constraint fk
            references piece
            on delete cascade,
-- 0 = image, 1 = audio, 2 = video
    type integer not null,
    data BLOB    not null
);

-- a geographic location
create table piece_3_location
(
    id   integer
        constraint pk
            primary key
        constraint fk
            references piece
            on delete cascade,
    name TEXT,
    lat  REAL not null,
    lon  REAL not null
);

-- this table is unique because the rows are many : 1 piece
-- a mapping of per piece, per activity, the value for the activity (eg 0 = not done, 1 = done)
-- very intentionally NOT a table with 1 column per activity, because the user can update the list of activities, and schema changes are expensive
create table piece_4_activities
(
    id          integer not null
        constraint piece_4_activities_piece_id_fk
            references piece
            on delete cascade,
    activity_id integer not null
        constraint piece_4_activities_activities_id_fk
            references activities
            on delete cascade,
    value       integer not null default 0,
    constraint piece_4_activities_pk
        primary key (id, activity_id)
-- sqlite doesnt distinguish between int and bool, and i might want to add int support in the future, so int is fine
);

-- predefined activities user can choose from for an entry
create table activities
(
    id    integer not null
        constraint activities_pk
            primary key,
    name  text    not null,
    -- maybe at some point i'll implement custom icons, but unicode emoji are fine for now
    emoji text    not null
    -- also might add order and categories in the future
);

-- key-value store for settings
create table settings
(
    key   string not null
        constraint settings_pk
            primary key,
    value ANY
);