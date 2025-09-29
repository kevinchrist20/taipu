import { ref, computed } from 'vue';
import LessonService from '../services/lesson.service';
import { SessionStore } from '../storage';
import useAlert from './useAlert';

// Global state for completed lessons (shared across components)
const completedLessonsCache = ref<number[]>([]);
const loading = ref(false);
const lastFetchTime = ref<number | null>(null);

const CACHE_DURATION = 3 * 60 * 1000;

export default function useCompletedLessons() {
  const completedLessons = computed(() => completedLessonsCache.value);
  
  const isDataFresh = computed(() => {
    if (!lastFetchTime.value) return false;
    return Date.now() - lastFetchTime.value < CACHE_DURATION;
  });

  const fetchCompletedLessons = async (forceRefresh = false) => {
    if (!forceRefresh && isDataFresh.value) {
      return completedLessonsCache.value;
    }

    try {
      loading.value = true;
      const user = SessionStore.user;
      
      if (!user) {
        throw new Error('User not found in session');
      }

      const fetchedCompletedLessons = await LessonService.getCompletedLessons(user.id);
      
      completedLessonsCache.value = fetchedCompletedLessons;
      lastFetchTime.value = Date.now();
      
      return fetchedCompletedLessons;
    } catch (error) {
      useAlert().setAlert({ 
        message: 'Error fetching completed lessons. Please try again later.', 
        type: 'danger' 
      });
      console.error('Error fetching completed lessons:', error);
      throw error;
    } finally {
      loading.value = false;
    }
  };

  const isLessonCompleted = (lessonId: number) => {
    return completedLessonsCache.value.includes(lessonId);
  };

  const markLessonCompleted = (lessonId: number) => {
    if (!isLessonCompleted(lessonId)) {
      completedLessonsCache.value.push(lessonId);
    }
  };

  const getCompletedLessonsCount = () => {
    return completedLessonsCache.value.length;
  };

  const clearCache = () => {
    completedLessonsCache.value = [];
    lastFetchTime.value = null;
  };

  const refreshCompletedLessons = async () => {
    return await fetchCompletedLessons(true);
  };

  return {
    completedLessons,
    loading: computed(() => loading.value),
    isDataFresh,
    fetchCompletedLessons,
    isLessonCompleted,
    markLessonCompleted,
    getCompletedLessonsCount,
    clearCache,
    refreshCompletedLessons
  };
}