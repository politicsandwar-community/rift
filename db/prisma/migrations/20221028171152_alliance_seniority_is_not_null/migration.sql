/*
  Warnings:

  - Made the column `alliance_seniority` on table `nations` required. This step will fail if there are existing NULL values in that column.

*/
-- AlterTable
ALTER TABLE "nations" ALTER COLUMN "alliance_seniority" SET NOT NULL;
