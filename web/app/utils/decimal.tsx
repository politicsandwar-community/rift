import type { DineroOptions } from "dinero.js";
import { dinero } from "dinero.js";

export function decimal(options: Omit<DineroOptions<number>, "currency">) {
  return dinero({
    ...options,
    currency: {
      code: "D",
      base: 10,
      exponent: 2,
    },
  });
}
