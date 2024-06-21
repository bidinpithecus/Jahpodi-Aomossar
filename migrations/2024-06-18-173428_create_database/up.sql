CREATE TABLE "user" (
    "id" INT PRIMARY KEY,
    "username" VARCHAR(50) NOT NULL,
    "email" VARCHAR(100) NOT NULL,
    "creation_date" DATE NOT NULL,
    "password" VARCHAR(100) NOT NULL
);

CREATE TABLE "admin" (
    "id" INT PRIMARY KEY,
    FOREIGN KEY ("id") REFERENCES "user"("id")
);

CREATE TABLE "regular_user" (
    "id" INT PRIMARY KEY,
    FOREIGN KEY ("id") REFERENCES "user"("id")
);

CREATE TABLE "ingredient" (
    "id" INT PRIMARY KEY,
    "name" VARCHAR(100)
);

CREATE TABLE "recipe" (
    "id" INT PRIMARY KEY,
    "title" VARCHAR(100) NOT NULL,
    "description" TEXT NOT NULL,
    "creation_date" DATE NOT NULL,
    "directions" TEXT NOT NULL
);

CREATE TABLE "ingredient_request" (
    "id" INT PRIMARY KEY,
    "name" VARCHAR(100) NOT NULL
);

CREATE TABLE "question" (
    "id" INT PRIMARY KEY,
    "title" VARCHAR(200) NOT NULL ,
    "body" TEXT NOT NULL,
    "date" DATE NOT NULL,
    "user_id" INT,
    FOREIGN KEY ("user_id") REFERENCES "regular_user"("id")
);

CREATE TABLE "answer" (
    "id" INT PRIMARY KEY,
    "body" TEXT NOT NULL,
    "date" DATE NOT NULL,
    "question_id" INT,
    "user_id" INT,
    FOREIGN KEY ("question_id") REFERENCES "question"("id"),
    FOREIGN KEY ("user_id") REFERENCES "regular_user"("id")
);

CREATE TABLE "rating" (
    "id" INT PRIMARY KEY,
    "score" INT NOT NULL,
    "recipe_id" INT,
    "user_id" INT,
    FOREIGN KEY ("recipe_id") REFERENCES "recipe"("id"),
    FOREIGN KEY ("user_id") REFERENCES "regular_user"("id")
);

CREATE TABLE "recipe_ingredient" (
    "recipe_id" INT,
    "ingredient_id" INT NOT NULL,
    "quantity" VARCHAR(50) NOT NULL,
    PRIMARY KEY ("ingredient_id", "recipe_id"),
    FOREIGN KEY ("ingredient_id") REFERENCES "ingredient"("id"),
    FOREIGN KEY ("recipe_id") REFERENCES "recipe"("id")
);

CREATE TABLE "recipe_author" (
    "user_id" INT,
    "recipe_id" INT,
    PRIMARY KEY ("user_id", "recipe_id"),
    FOREIGN KEY ("user_id") REFERENCES "regular_user"("id"),
    FOREIGN KEY ("recipe_id") REFERENCES "recipe"("id")
);

CREATE TABLE "user_recipe" (
    "user_id" INT,
    "recipe_id" INT,
    PRIMARY KEY ("user_id", "recipe_id"),
    FOREIGN KEY ("user_id") REFERENCES "regular_user"("id"),
    FOREIGN KEY ("recipe_id") REFERENCES "recipe"("id")
);

CREATE TABLE "user_question" (
    "user_id" INT,
    "question_id" INT,
    PRIMARY KEY ("user_id", "question_id"),
    FOREIGN KEY ("user_id") REFERENCES "regular_user"("id"),
    FOREIGN KEY ("question_id") REFERENCES "question"("id")
);

CREATE TABLE "user_ingredient_request" (
    "user_id" INT,
    "request_id" INT,
    PRIMARY KEY ("user_id", "request_id"),
    FOREIGN KEY ("user_id") REFERENCES "regular_user"("id"),
    FOREIGN KEY ("request_id") REFERENCES "ingredient_request"("id")
);

CREATE TABLE "question_answer" (
    "question_id" INT,
    "answer_id" INT,
    PRIMARY KEY ("question_id", "answer_id"),
    FOREIGN KEY ("question_id") REFERENCES "question"("id"),
    FOREIGN KEY ("answer_id") REFERENCES "answer"("id")
);
