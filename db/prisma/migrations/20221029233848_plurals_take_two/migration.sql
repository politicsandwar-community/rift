/*
  Warnings:

  - You are about to drop the column `bauxite_mine` on the `cities` table. All the data in the column will be lost.
  - You are about to drop the column `oil_well` on the `cities` table. All the data in the column will be lost.
  - You are about to drop the column `uranium_mine` on the `cities` table. All the data in the column will be lost.
  - Added the required column `bauxite_mines` to the `cities` table without a default value. This is not possible if the table is not empty.
  - Added the required column `oil_wells` to the `cities` table without a default value. This is not possible if the table is not empty.
  - Added the required column `uranium_mines` to the `cities` table without a default value. This is not possible if the table is not empty.

*/
-- AlterTable
ALTER TABLE "cities" DROP COLUMN "bauxite_mine",
DROP COLUMN "oil_well",
DROP COLUMN "uranium_mine",
ADD COLUMN     "bauxite_mines" INTEGER NOT NULL,
ADD COLUMN     "oil_wells" INTEGER NOT NULL,
ADD COLUMN     "uranium_mines" INTEGER NOT NULL;
