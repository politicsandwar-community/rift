/*
  Warnings:

  - Made the column `owner_id` on table `target_configs` required. This step will fail if there are existing NULL values in that column.
  - Made the column `name` on table `target_configs` required. This step will fail if there are existing NULL values in that column.
  - Made the column `count` on table `target_configs` required. This step will fail if there are existing NULL values in that column.
  - Made the column `rater` on table `target_configs` required. This step will fail if there are existing NULL values in that column.
  - Made the column `condition` on table `target_configs` required. This step will fail if there are existing NULL values in that column.
  - Made the column `use_condition` on table `target_configs` required. This step will fail if there are existing NULL values in that column.
  - Made the column `attack` on table `target_configs` required. This step will fail if there are existing NULL values in that column.
  - Made the column `cities` on table `target_raters` required. This step will fail if there are existing NULL values in that column.
  - Made the column `infrastructure` on table `target_raters` required. This step will fail if there are existing NULL values in that column.
  - Made the column `activity` on table `target_raters` required. This step will fail if there are existing NULL values in that column.
  - Made the column `soldiers` on table `target_raters` required. This step will fail if there are existing NULL values in that column.
  - Made the column `tanks` on table `target_raters` required. This step will fail if there are existing NULL values in that column.
  - Made the column `aircraft` on table `target_raters` required. This step will fail if there are existing NULL values in that column.
  - Made the column `ships` on table `target_raters` required. This step will fail if there are existing NULL values in that column.
  - Made the column `missiles` on table `target_raters` required. This step will fail if there are existing NULL values in that column.
  - Made the column `nukes` on table `target_raters` required. This step will fail if there are existing NULL values in that column.
  - Made the column `money` on table `target_raters` required. This step will fail if there are existing NULL values in that column.
  - Made the column `coal` on table `target_raters` required. This step will fail if there are existing NULL values in that column.
  - Made the column `oil` on table `target_raters` required. This step will fail if there are existing NULL values in that column.
  - Made the column `uranium` on table `target_raters` required. This step will fail if there are existing NULL values in that column.
  - Made the column `iron` on table `target_raters` required. This step will fail if there are existing NULL values in that column.
  - Made the column `bauxite` on table `target_raters` required. This step will fail if there are existing NULL values in that column.
  - Made the column `lead` on table `target_raters` required. This step will fail if there are existing NULL values in that column.
  - Made the column `gasoline` on table `target_raters` required. This step will fail if there are existing NULL values in that column.
  - Made the column `munitions` on table `target_raters` required. This step will fail if there are existing NULL values in that column.
  - Made the column `steel` on table `target_raters` required. This step will fail if there are existing NULL values in that column.
  - Made the column `aluminum` on table `target_raters` required. This step will fail if there are existing NULL values in that column.
  - Made the column `food` on table `target_raters` required. This step will fail if there are existing NULL values in that column.

*/
-- AlterTable
ALTER TABLE "target_configs" ALTER COLUMN "owner_id" SET NOT NULL,
ALTER COLUMN "name" SET NOT NULL,
ALTER COLUMN "count" SET NOT NULL,
ALTER COLUMN "rater" SET NOT NULL,
ALTER COLUMN "condition" SET NOT NULL,
ALTER COLUMN "use_condition" SET NOT NULL,
ALTER COLUMN "attack" SET NOT NULL;

-- AlterTable
ALTER TABLE "target_raters" ALTER COLUMN "cities" SET NOT NULL,
ALTER COLUMN "infrastructure" SET NOT NULL,
ALTER COLUMN "activity" SET NOT NULL,
ALTER COLUMN "soldiers" SET NOT NULL,
ALTER COLUMN "tanks" SET NOT NULL,
ALTER COLUMN "aircraft" SET NOT NULL,
ALTER COLUMN "ships" SET NOT NULL,
ALTER COLUMN "missiles" SET NOT NULL,
ALTER COLUMN "nukes" SET NOT NULL,
ALTER COLUMN "money" SET NOT NULL,
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
ALTER COLUMN "food" SET NOT NULL;
