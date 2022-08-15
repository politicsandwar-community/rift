/* eslint-disable @typescript-eslint/no-var-requires */
// eslint-disable-next-line no-undef
const { withEsbuildOverride } = require("remix-esbuild-override");
const GlobalsPolyfills =
  // eslint-disable-next-line no-undef
  require("@esbuild-plugins/node-globals-polyfill").default;

withEsbuildOverride((option, { isServer }) => {
  if (isServer)
    option.plugins = [
      GlobalsPolyfills({
        buffer: true,
      }),
      ...option.plugins,
    ];

  return option;
});

/** @type {import('@remix-run/dev').AppConfig} */
// eslint-disable-next-line no-undef
module.exports = {
  serverBuildTarget: "cloudflare-pages",
  server: "./server.js",
  devServerBroadcastDelay: 1000,
  ignoredRouteFiles: ["**/.*"],
  // appDirectory: "app",
  // assetsBuildDirectory: "public/build",
  // serverBuildPath: "functions/[[path]].js",
  // publicPath: "/build/",
};
