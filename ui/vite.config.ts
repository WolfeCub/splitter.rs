import { PluginOption, defineConfig } from 'vite'
import react from '@vitejs/plugin-react-swc'
import { execSync } from 'child_process'

const protoPlugin: PluginOption = {
    name: 'proto-plugin',
    buildStart(_options) {
        // execSync('npx protoc --ts_out src/proto --proto_path ../proto ../proto/splitter.proto')
        execSync('pnpm exec protoc --ts_out src/proto --proto_path ../proto ../proto/splitter.proto')
    },
};

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react(), protoPlugin],
})
