import createMDX from '@next/mdx'
import { ReactCompilerRuntime } from 'next/dist/server/route-modules/app-page/vendored/rsc/entrypoints'
 
/** @type {import('next').NextConfig} */
const nextConfig = {
  // Configure `pageExtensions` to include markdown and MDX files
  pageExtensions: ['js', 'jsx', 'md', 'mdx', 'ts', 'tsx'],
}
 
const withMDX = createMDX({
  extension: /\.(md|mdx)$/,
})
 
// Merge MDX config with Next.js config
export default withMDX(nextConfig)