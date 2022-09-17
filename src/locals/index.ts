import { createI18n } from 'vue-i18n'
import { changeLang, getConfigs } from '~/serviece/client/configs'

import zh from './zh'
import en from './en'

const messages = {
  en,
  zh,
}

const language = (navigator.language || 'zh').toLocaleLowerCase() // 这是获取浏览器的语言

const i18n: any = createI18n({
  locale: language.split('-')[0] || 'zh', // 首先从缓存里拿，没有的话就用浏览器语言，
  fallbackLocale: 'zh', // 设置备用语言
  messages,
})

export const changeClientLang = async (lang: 'zh' | 'en' = 'zh') => {
  try {
    i18n.global.locale = lang
    await changeLang(lang)
  } catch (error) {
    localStorage.setItem('lang', lang)
  }
}
;(async () => {
  let configs: any

  try {
    configs = await getConfigs()
  } catch (error) {
    configs = {
      lang: localStorage.getItem('lang'),
    }
  }

  if (configs.lang) {
    changeClientLang(configs.lang)
  }
})()

export default i18n
