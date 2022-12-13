/*
  Warnings:

  - Added the required column `owner_id` to the `target_raters` table without a default value. This is not possible if the table is not empty.
  - Added the required column `use_condition` to the `target_raters` table without a default value. This is not possible if the table is not empty.

*/
-- AlterTable
ALTER TABLE "target_raters" ADD COLUMN     "owner_id" BIGINT NOT NULL,
ADD COLUMN     "use_condition" TEXT NOT NULL;
