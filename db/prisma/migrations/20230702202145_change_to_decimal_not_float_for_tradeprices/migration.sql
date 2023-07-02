/*
  Warnings:

  - You are about to alter the column `coal` on the `tradeprices` table. The data in that column could be lost. The data in that column will be cast from `DoublePrecision` to `Decimal`.
  - You are about to alter the column `oil` on the `tradeprices` table. The data in that column could be lost. The data in that column will be cast from `DoublePrecision` to `Decimal`.
  - You are about to alter the column `uranium` on the `tradeprices` table. The data in that column could be lost. The data in that column will be cast from `DoublePrecision` to `Decimal`.
  - You are about to alter the column `iron` on the `tradeprices` table. The data in that column could be lost. The data in that column will be cast from `DoublePrecision` to `Decimal`.
  - You are about to alter the column `bauxite` on the `tradeprices` table. The data in that column could be lost. The data in that column will be cast from `DoublePrecision` to `Decimal`.
  - You are about to alter the column `lead` on the `tradeprices` table. The data in that column could be lost. The data in that column will be cast from `DoublePrecision` to `Decimal`.
  - You are about to alter the column `gasoline` on the `tradeprices` table. The data in that column could be lost. The data in that column will be cast from `DoublePrecision` to `Decimal`.
  - You are about to alter the column `munitions` on the `tradeprices` table. The data in that column could be lost. The data in that column will be cast from `DoublePrecision` to `Decimal`.
  - You are about to alter the column `steel` on the `tradeprices` table. The data in that column could be lost. The data in that column will be cast from `DoublePrecision` to `Decimal`.
  - You are about to alter the column `aluminum` on the `tradeprices` table. The data in that column could be lost. The data in that column will be cast from `DoublePrecision` to `Decimal`.
  - You are about to alter the column `food` on the `tradeprices` table. The data in that column could be lost. The data in that column will be cast from `DoublePrecision` to `Decimal`.
  - You are about to alter the column `credits` on the `tradeprices` table. The data in that column could be lost. The data in that column will be cast from `DoublePrecision` to `Decimal`.

*/
-- AlterTable
ALTER TABLE "tradeprices" ALTER COLUMN "coal" SET DATA TYPE DECIMAL,
ALTER COLUMN "oil" SET DATA TYPE DECIMAL,
ALTER COLUMN "uranium" SET DATA TYPE DECIMAL,
ALTER COLUMN "iron" SET DATA TYPE DECIMAL,
ALTER COLUMN "bauxite" SET DATA TYPE DECIMAL,
ALTER COLUMN "lead" SET DATA TYPE DECIMAL,
ALTER COLUMN "gasoline" SET DATA TYPE DECIMAL,
ALTER COLUMN "munitions" SET DATA TYPE DECIMAL,
ALTER COLUMN "steel" SET DATA TYPE DECIMAL,
ALTER COLUMN "aluminum" SET DATA TYPE DECIMAL,
ALTER COLUMN "food" SET DATA TYPE DECIMAL,
ALTER COLUMN "credits" SET DATA TYPE DECIMAL;
