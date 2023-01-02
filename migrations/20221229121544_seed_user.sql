-- Add migration script here
INSERT INTO users (user_id, username, password_hash)
VALUES (
           'a8379ce3-f97c-4d33-94f6-c5dcbe616c31',
           'admin',
            -- password: everythinghastostartsomewhere
           '$argon2id$v=19$m=15000,t=2,p=1$wfV5wxXr+5IU7BykASghlg$0ibTr9oUtuYf6pzrB1Vw7Rftt5mBTktaXP6K/vp3LH8'
       );
