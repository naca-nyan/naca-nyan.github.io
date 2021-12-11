import { createApp } from 'vue'
import App from './App.vue'
import router from './router'

const title_suffix = "naca-nyan.github.io"

router.beforeEach((to, from, next) => {
    document.title = to.meta.title + " - " + title_suffix
    next()
})

createApp(App).use(router).mount('#app')
