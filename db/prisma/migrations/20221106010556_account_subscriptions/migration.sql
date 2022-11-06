/*
  Warnings:

  - Added the required column `last_active` to the `nations` table without a default value. This is not possible if the table is not empty.

*/
-- AlterTable
ALTER TABLE "nations" ADD COLUMN     "discord_id" TEXT,
ADD COLUMN     "last_active" TIMESTAMPTZ(6) NOT NULL;
