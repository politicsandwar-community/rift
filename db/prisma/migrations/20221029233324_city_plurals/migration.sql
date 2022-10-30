/*
  Warnings:

  - You are about to drop the column `bauxite_mine` on the `builds` table. All the data in the column will be lost.
  - You are about to drop the column `oil_well` on the `builds` table. All the data in the column will be lost.
  - You are about to drop the column `uranium_mine` on the `builds` table. All the data in the column will be lost.

*/
-- AlterTable
ALTER TABLE "builds" DROP COLUMN "bauxite_mine",
DROP COLUMN "oil_well",
DROP COLUMN "uranium_mine",
ADD COLUMN     "bauxite_mines" INTEGER,
ADD COLUMN     "oil_wells" INTEGER,
ADD COLUMN     "uranium_mines" INTEGER;
