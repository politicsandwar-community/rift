/*
  Warnings:

  - Made the column `id` on table `tradeprices` required. This step will fail if there are existing NULL values in that column.

*/
-- AlterTable
ALTER TABLE "tradeprices" ALTER COLUMN "id" SET NOT NULL;
