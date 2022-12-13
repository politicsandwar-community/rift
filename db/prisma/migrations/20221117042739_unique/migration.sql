/*
  Warnings:

  - A unique constraint covering the columns `[id]` on the table `target_configs` will be added. If there are existing duplicate values, this will fail.
  - A unique constraint covering the columns `[id]` on the table `target_raters` will be added. If there are existing duplicate values, this will fail.

*/
-- CreateIndex
CREATE UNIQUE INDEX "target_configs_id_key" ON "target_configs"("id");

-- CreateIndex
CREATE UNIQUE INDEX "target_raters_id_key" ON "target_raters"("id");
