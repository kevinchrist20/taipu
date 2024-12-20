export interface Alert {
    message: string
    type: 'warning' | 'danger' | 'info' | 'neutral' | 'success'
    priority?: boolean
    timer?: number
    position?: number
    explicit?: boolean
    action?: string
    title?: string
}