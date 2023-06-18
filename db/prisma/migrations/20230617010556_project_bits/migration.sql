/*
  Warnings:

  - Added the required column `last_active` to the `nations` table without a default value. This is not possible if the table is not empty.

*/
-- AlterTable
ALTER TABLE "nations" ADD COLUMN     "project_bits" BIGINT NOT NULL;
