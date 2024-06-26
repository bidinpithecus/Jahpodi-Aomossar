CREATE TABLE "user" (
    "id" SERIAL PRIMARY KEY,
    "username" VARCHAR(50) UNIQUE NOT NULL,
    "email" VARCHAR(100) UNIQUE NOT NULL,
    "creation_time" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "password" VARCHAR(100) NOT NULL
);

CREATE TABLE "admin" (
    "id" SERIAL PRIMARY KEY,
    FOREIGN KEY ("id") REFERENCES "user"("id")
);

CREATE TABLE "regular_user" (
    "id" SERIAL PRIMARY KEY,
    FOREIGN KEY ("id") REFERENCES "user"("id")
);

CREATE TABLE "ingredient" (
    "id" SERIAL PRIMARY KEY,
    "name" VARCHAR(100) UNIQUE NOT NULL
);

CREATE TABLE "recipe" (
    "id" SERIAL PRIMARY KEY,
    "title" VARCHAR(100) NOT NULL,
    "description" TEXT NOT NULL,
    "creation_date" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "directions" TEXT NOT NULL,
    "user_id" INT NOT NULL,

    FOREIGN KEY ("user_id") REFERENCES "regular_user"("id")
);

CREATE TABLE "ingredient_request" (
    "id" SERIAL PRIMARY KEY,
    "name" VARCHAR(100) NOT NULL
);

CREATE TABLE "question" (
    "id" SERIAL PRIMARY KEY,
    "title" VARCHAR(200) NOT NULL ,
    "body" TEXT NOT NULL,
    "date" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "user_id" INT NOT NULL,
    "recipe_id" INT NOT NULL,

    FOREIGN KEY ("recipe_id") REFERENCES "recipe"("id"),
    FOREIGN KEY ("user_id") REFERENCES "regular_user"("id")
);

CREATE TABLE "answer" (
    "id" SERIAL PRIMARY KEY,
    "body" TEXT NOT NULL,
    "date" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "question_id" INT NOT NULL,
    "user_id" INT NOT NULL,

    FOREIGN KEY ("question_id") REFERENCES "question"("id"),
    FOREIGN KEY ("user_id") REFERENCES "regular_user"("id")
);

CREATE TABLE "rating" (
    "score" INT NOT NULL,
    "recipe_id" INT NOT NULL,
    "user_id" INT NOT NULL,

    PRIMARY KEY ("recipe_id", "user_id"),
    FOREIGN KEY ("recipe_id") REFERENCES "recipe"("id"),
    FOREIGN KEY ("user_id") REFERENCES "regular_user"("id")
);

CREATE TABLE "recipe_ingredient" (
    "recipe_id" INT NOT NULL,
    "ingredient_id" INT NOT NULL,
    "quantity" VARCHAR(50) NOT NULL,

    PRIMARY KEY ("ingredient_id", "recipe_id"),
    FOREIGN KEY ("ingredient_id") REFERENCES "ingredient"("id"),
    FOREIGN KEY ("recipe_id") REFERENCES "recipe"("id")
);
