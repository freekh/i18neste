/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,
  swcMinify: false,
  experimental: {
    swcPlugins: [["swc-i18neste-plugin", {}]],
  },
};

module.exports = nextConfig;
