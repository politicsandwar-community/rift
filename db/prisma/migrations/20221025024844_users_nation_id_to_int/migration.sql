/*
  Warnings:

  - You are about to alter the column `nation_id` on the `users` table. The data in that column could be lost. The data in that column will be cast from `BigInt` to `Integer`.

*/
-- AlterTable
ALTER TABLE "users" ALTER COLUMN "nation_id" SET DATA TYPE INTEGER;
