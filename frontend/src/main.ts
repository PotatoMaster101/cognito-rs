import { createApp } from "vue"
import { createPinia } from "pinia"
import App from "./App.vue"
import router from "./router.ts"
import { userManager } from "./auth"

const app = createApp(App)
app.use(createPinia())
app.use(router)
app.provide("userManager", userManager)
app.mount("#app")
