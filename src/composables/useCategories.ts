import { ref, computed } from 'vue';
import LessonService from '../services/lesson.service';
import { SessionStore } from '../storage';
import { CategoryWithLessons } from '../types/bindings';
import { formatCategoryName } from '../utils/ui-formatters';
import useAlert from './useAlert';

// Global state for categories (shared across components)
const categoriesCache = ref<CategoryWithLessons[]>([]);
const loading = ref(false);
const lastFetchTime = ref<number | null>(null);
const cachedUserId = ref<number | null>(null);
const cachedDifficulty = ref<string | null>(null);

// Cache duration: 5 minutes
const CACHE_DURATION = 5 * 60 * 1000;

export default function useCategories() {
  const categories = computed(() => categoriesCache.value);
  
  const isDataFresh = computed(() => {
    if (!lastFetchTime.value) return false;
    return Date.now() - lastFetchTime.value < CACHE_DURATION;
  });

  const fetchCategories = async (forceRefresh = false) => {
    try {
      loading.value = true;
      const user = SessionStore.user;
      
      if (!user) {
        throw new Error('User not found in session');
      }

      const currentDifficulty = user.lessonDifficulty || '';

      const cacheKeyChanged = cachedUserId.value !== user.id || cachedDifficulty.value !== currentDifficulty;
      if (cacheKeyChanged) {
        categoriesCache.value = [];
        lastFetchTime.value = null;
      }

      if (!forceRefresh && !cacheKeyChanged && isDataFresh.value && categoriesCache.value.length > 0) {
        return categoriesCache.value;
      }

      const fetchedCategories = await LessonService.getLessonsByCategories(
        currentDifficulty,
        user.id
      );
      
      categoriesCache.value = fetchedCategories;
      lastFetchTime.value = Date.now();
      cachedUserId.value = user.id;
      cachedDifficulty.value = currentDifficulty;
      
      return fetchedCategories;
    } catch (error) {
      useAlert().setAlert({ 
        message: 'Error fetching lessons. Please try again later.', 
        type: 'danger' 
      });
      console.error('Error fetching categories:', error);
      throw error;
    } finally {
      loading.value = false;
    }
  };

  const clearCache = () => {
    categoriesCache.value = [];
    lastFetchTime.value = null;
    cachedUserId.value = null;
    cachedDifficulty.value = null;
  };

  const getCategoryByName = (categoryName: string) => {
    return categoriesCache.value.find(cat => cat.category === categoryName);
  };

  const CATEGORY_DESCRIPTIONS: Record<string, string> = {
    'home-left':      'Master the left-hand home row keys: A, S, D, F',
    'home-right':     'Master the right-hand home row keys: J, K, L, ;',
    'home-combined':  'Combine both hands on the home row for fluid typing',
    'top-row':        'Learn the top row keys: Q W E R T Y U I O P',
    'bottom-row':     'Practice the bottom row keys: Z X C V B N M',
    'row-transitions':'Smooth transitions between all three rows',
    'punctuation':    'Add punctuation marks to your typing repertoire',
    'common-words':   'Type the most frequently used words in English',
    'numbers-row':    'Master the number keys across the top of the keyboard',
    'speed-drills':   'Push your WPM with rapid-fire speed drills',
  };

  const getCategoryDescription = (categoryName: string): string => {
    return CATEGORY_DESCRIPTIONS[categoryName] ?? '';
  };

  return {
    categories,
    loading: computed(() => loading.value),
    isDataFresh,
    fetchCategories,
    formatCategoryName,
    clearCache,
    getCategoryByName,
    getCategoryDescription,
  };
}