import { useDark, useToggle } from '@vueuse/core'
import { changeTheme } from '~/serviece/client/configs'

export const isDark = useDark({
  async onChanged(isDark) {
    let theme: any = isDark ? 'dark' : ''
    await changeTheme(theme)
  },
})
export const toggleDark = useToggle(isDark)
