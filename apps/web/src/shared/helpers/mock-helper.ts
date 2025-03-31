export async function delay(ms: number, callback?: () => void) {
  return new Promise((resolve) => {
    setTimeout(() => {
      if (callback) callback()
      resolve(true)
    }, ms)
  })
}
