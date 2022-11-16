/*
  Warnings:

  - Made the column `owner_type` on table `mentions` required. This step will fail if there are existing NULL values in that column.

*/
-- AlterTable
ALTER TABLE "mentions" ALTER COLUMN "owner_type" SET NOT NULL;
