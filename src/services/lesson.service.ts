import { CategoryWithLessons, Lesson } from '../types/bindings';
import { invoke } from '../types/tauri_commands';

export default {
    async getLessons(difficulty: string): Promise<Lesson[]> {
        const lessons = await invoke('get_lessons_by_difficulty', {
            difficulty: difficulty.toUpperCase()
        });
        return lessons;
    },

    async completeLesson(
        userId: number,
        lessonId: number,
        wpm: number,
        accuracy: number,
        grade: string,
        durationSeconds: number
    ): Promise<void> {
        await invoke('complete_lesson', {
            userId,
            lessonId,
            wpm,
            accuracy,
            grade,
            durationSeconds
        });
    },

    async getCompletedLessons(userId: number): Promise<number[]> {
        return await invoke('get_completed_lessons', { userId });
    },

    async getTestForLesson(lessonId: number): Promise<Lesson[]> {
        return await invoke('get_lesson_tests', { id: lessonId });
    },

    async getLessonsByCategories(
        difficulty: string,
        userId: number
    ): Promise<CategoryWithLessons[]> {
        return await invoke('get_lessons_by_categories', {
            difficulty: difficulty.toUpperCase(),
            userId
        });
    },

    async getCategoryTests(category: string, difficulty: string): Promise<Lesson[]> {
        return await invoke('get_category_tests', {
            category,
            difficulty: difficulty.toUpperCase()
        });
    },

    async getLessonById(lessonId: number): Promise<Lesson | null> {
        return await invoke('get_lesson_by_id', { lessonId: lessonId });
    }
}