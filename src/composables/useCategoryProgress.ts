import { ComputedRef } from 'vue';
import { CategoryWithLessons } from '../types/bindings';
import { TestStatus } from '../types';

export default function useCategoryProgress(completedLessonsIds: ComputedRef<number[]>) {
  
  const getCategoryProgress = (category: CategoryWithLessons) => {
    const totalLessons = category.lessons.length;
    const completedCount = category.lessons.filter(lesson =>
      completedLessonsIds.value.includes(lesson.id)
    ).length;
    const percentage = totalLessons > 0 ? Math.round((completedCount / totalLessons) * 100) : 0;

    return {
      completed: completedCount,
      total: totalLessons,
      percentage
    };
  };

  const isCategoryTestCompleted = (category: CategoryWithLessons) => {
    return category.tests.some(test => completedLessonsIds.value.includes(test.id));
  };

  const getCategoryTestStatus = (category: CategoryWithLessons): TestStatus => {
    if (!category.isAvailable) return 'Locked';

    const allLessonsCompleted = category.lessons.every(lesson =>
      completedLessonsIds.value.includes(lesson.id)
    );

    if (!allLessonsCompleted) return 'Locked';
    if (isCategoryTestCompleted(category)) return 'Passed';
    return 'Ready';
  };

  const isCategoryCompleted = (category: CategoryWithLessons) => {
    return getCategoryProgress(category).percentage === 100;
  };

  const getAvailableCategories = (categories: CategoryWithLessons[]) => {
    return categories.filter(category => category.isAvailable);
  };

  const getLockedCategories = (categories: CategoryWithLessons[]) => {
    return categories.filter(category => !category.isAvailable);
  };

  const getCategoriesWithProgress = (categories: CategoryWithLessons[]) => {
    return categories.map(category => ({
      ...category,
      progress: getCategoryProgress(category),
      testStatus: getCategoryTestStatus(category),
      isCompleted: isCategoryCompleted(category)
    }));
  };

  return {
    getCategoryProgress,
    isCategoryTestCompleted,
    getCategoryTestStatus,
    isCategoryCompleted,
    getAvailableCategories,
    getLockedCategories,
    getCategoriesWithProgress
  };
}