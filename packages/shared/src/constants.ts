export enum PLATFORM_TYPE {
  MINI = 'mini',
  WEB = 'web',
  RN = 'rn',
  HARMONY = 'harmony',
  QUICK = 'quickapp',
}

export const COMPILE_MODE_IDENTIFIER_PREFIX = 'f'

export const COMPILE_MODE_SUB_RENDER_FN = 'subRenderFn'

export const PLATFORM_CONFIG_MAP = {
  h5: {
    type: PLATFORM_TYPE.WEB
  },
  harmony: {
    type: PLATFORM_TYPE.HARMONY
  },
  mini: {
    type: PLATFORM_TYPE.MINI
  },
  rn: {
    type: PLATFORM_TYPE.RN
  },
  quickapp: {
    type: PLATFORM_TYPE.QUICK
  },
}
