import { ref, computed } from 'vue';
import LessonService from '../services/lesson.service';
import { SessionStore } from '../storage';
import { CategoryWithLessons } from '../types/bindings';
import useAlert from './useAlert';

// Global state for categories (shared across components)
const categoriesCache = ref<CategoryWithLessons[]>([]);
const loading = ref(false);
const lastFetchTime = ref<number | null>(null);
const cachedUserId = ref<number | null>(null);

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

      const userChanged = cachedUserId.value !== user.id;
      if (userChanged) {
        categoriesCache.value = [];
        lastFetchTime.value = null;
      }

      if (!forceRefresh && !userChanged && isDataFresh.value && categoriesCache.value.length > 0) {
        return categoriesCache.value;
      }

      const fetchedCategories = await LessonService.getLessonsByCategories(
        user.lessonDifficulty || '',
        user.id
      );
      
      categoriesCache.value = fetchedCategories;
      lastFetchTime.value = Date.now();
      cachedUserId.value = user.id;
      
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

  const formatCategoryName = (categoryName: string) => {
    return categoryName
      .split('-')
      .map(word => word.charAt(0).toUpperCase() + word.slice(1))
      .join(' ');
  };

  const clearCache = () => {
    categoriesCache.value = [];
    lastFetchTime.value = null;
    cachedUserId.value = null;
  };

  const getCategoryByName = (categoryName: string) => {
    return categoriesCache.value.find(cat => cat.category === categoryName);
  };

  const CATEGORY_DESCRIPTIONS: Record<string, string> = {
    'home-left':           'Master the left-hand home row keys: A, S, D, F',
    'home-right':          'Master the right-hand home row keys: J, K, L, ;',
    'home-combined':       'Combine both hands on the home row for fluid typing',
    'transition-top-home': 'Smooth transitions between the top and home rows',
    'transition-home-bottom': 'Smooth transitions between the home and bottom rows',
    'top-left':            'Learn the top-left row keys: Q, W, E, R',
    'top-right':           'Learn the top-right row keys: U, I, O, P',
    'top-home-combined':   'Combine the top row with the home row',
    'bottom-left':         'Practice the bottom-left row keys: Z, X, C, V',
    'bottom-right':        'Practice the bottom-right row keys: N, M, , .',
    'punctuation':         'Add punctuation marks to your typing repertoire',
    'full-keyboard':       'Master the full keyboard with real words and sentences',
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