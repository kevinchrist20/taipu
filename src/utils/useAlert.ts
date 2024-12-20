import { computed, reactive, ref } from "vue"
import { Alert } from "../types"

const alerts = reactive(new Set<Alert>([]))
const currentAlert = computed(() => Array.from(alerts)[0])
const showToast = ref(false)

export default function () {
  return {
    setAlert,
    showToast,
    currentAlert,
    alerts,
    reset,
    clearCurrentAlert,
    success,
    danger,
  }
}

function setAlert(payload: Alert) {
  // if (new Set(alerts).has(payload))
  //   return
  alerts.clear()
  payload.position = alerts.size + 1
  payload.action = payload.action ?? 'Close'
  alerts.add(payload)
  showToast.value = true
}

function success(message: string, title?: string) {
  setAlert({ type: 'success', title, message })
}

function danger(message: string, title?: string) {
  setAlert({ type: 'danger', title, message })
}

function reset() {
  alerts.clear()
  showToast.value = false
}

function clearCurrentAlert() {
  if (currentAlert.value) {
    alerts.delete(currentAlert.value)
    alerts.forEach(alert => alert.position = alert.position! + 1)
  }
}
