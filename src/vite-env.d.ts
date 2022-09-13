/// <reference types="vite/client" />

declare module 'marked'
declare module 'vconsole'

declare module '*.vue' {
  import type { DefineComponent } from 'vue'
  const component: DefineComponent<{}, {}, any>
  export default component
}
