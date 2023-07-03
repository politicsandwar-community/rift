/*
  Warnings:

  - Made the column `date` on table `tradeprices` required. This step will fail if there are existing NULL values in that column.
  - Made the column `coal` on table `tradeprices` required. This step will fail if there are existing NULL values in that column.
  - Made the column `oil` on table `tradeprices` required. This step will fail if there are existing NULL values in that column.
  - Made the column `uranium` on table `tradeprices` required. This step will fail if there are existing NULL values in that column.
  - Made the column `iron` on table `tradeprices` required. This step will fail if there are existing NULL values in that column.
  - Made the column `bauxite` on table `tradeprices` required. This step will fail if there are existing NULL values in that column.
  - Made the column `lead` on table `tradeprices` required. This step will fail if there are existing NULL values in that column.
  - Made the column `gasoline` on table `tradeprices` required. This step will fail if there are existing NULL values in that column.
  - Made the column `munitions` on table `tradeprices` required. This step will fail if there are existing NULL values in that column.
  - Made the column `steel` on table `tradeprices` required. This step will fail if there are existing NULL values in that column.
  - Made the column `aluminum` on table `tradeprices` required. This step will fail if there are existing NULL values in that column.
  - Made the column `food` on table `tradeprices` required. This step will fail if there are existing NULL values in that column.
  - Made the column `credits` on table `tradeprices` required. This step will fail if there are existing NULL values in that column.

*/
-- AlterTable
ALTER TABLE "tradeprices" ALTER COLUMN "date" SET NOT NULL,
ALTER COLUMN "coal" SET NOT NULL,
ALTER COLUMN "oil" SET NOT NULL,
ALTER COLUMN "uranium" SET NOT NULL,
ALTER COLUMN "iron" SET NOT NULL,
ALTER COLUMN "bauxite" SET NOT NULL,
ALTER COLUMN "lead" SET NOT NULL,
ALTER COLUMN "gasoline" SET NOT NULL,
ALTER COLUMN "munitions" SET NOT NULL,
ALTER COLUMN "steel" SET NOT NULL,
ALTER COLUMN "aluminum" SET NOT NULL,
ALTER COLUMN "food" SET NOT NULL,
ALTER COLUMN "credits" SET NOT NULL;
