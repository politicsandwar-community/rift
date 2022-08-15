import { useMantineTheme } from "@mantine/core";
import { useMediaQuery } from "@mantine/hooks";

export function useOnMobile() {
  const theme = useMantineTheme();
  return useMediaQuery(`(max-width: ${theme.breakpoints.sm}px)`);
}
