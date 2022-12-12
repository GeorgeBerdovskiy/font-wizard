import { createRouter, createWebHistory } from 'vue-router'

import SearchFonts from '../Components/Pages/SearchFonts.vue'
import SystemFonts from '../Components/Pages/SystemFonts.vue'
import FontBoxes from '../Components/Pages/FontBoxes.vue'
import Settings from '../Components/Pages/Settings.vue'

const routes = [
    {
        path: '/',
        component: SearchFonts
    },
	{
        path: '/system-fonts',
        component: SystemFonts,
    },
	{
        path: '/font-boxes',
        component: FontBoxes,
    },
	{
        path: '/settings',
        component: Settings,
    }
]

const router = createRouter({
    history: createWebHistory(),
    routes
})

export default router