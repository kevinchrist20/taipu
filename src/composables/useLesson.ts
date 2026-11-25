import type { Lesson } from '../types/bindings'
import { computed, ref } from 'vue'
import lessonService from '../services/lesson.service'
import useAlert from './useAlert'

const currentLesson = ref<Lesson | null>(null)
const loading = ref(false)

export default function useLesson() {
  const getLessonById = async (lessonId: number): Promise<Lesson | null> => {
    try {
      loading.value = true

      return await lessonService.getLessonById(lessonId)
    }
    catch (error) {
      useAlert().setAlert({
        message: 'Error fetching lesson. Please try again later.',
        type: 'danger',
      })
      console.error('Error fetching lesson:', error)
      return null
    }
    finally {
      loading.value = false
    }
  }

  const fetchLesson = async (lessonId: number) => {
    const lesson = await getLessonById(lessonId)
    currentLesson.value = lesson
    return lesson
  }

  const clearLesson = () => {
    currentLesson.value = null
  }

  return {
    currentLesson: computed(() => currentLesson.value),
    loading: computed(() => loading.value),
    fetchLesson,
    getLessonById,
    clearLesson,
  }
}
