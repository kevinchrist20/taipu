import { Lesson, LessonTest } from '../types/bindings';
import { invoke } from '../types/tauri_commands';

export default {
    async getLessons(difficulty: string): Promise<Lesson[]> {
        const lessons = await invoke('get_lessons_by_difficulty', { 
            difficulty: difficulty.toUpperCase() 
        });
        return lessons;
    },
    
    async completeLesson(userId: number, lessonId: number): Promise<void> {
        await invoke('complete_lesson', { userId, lessonId });
    },
    
    async getCompletedLessons(userId: number): Promise<number[]> {
        return await invoke('get_completed_lessons', { userId });
    },
    
    async getTestForLesson(lessonId: number): Promise<LessonTest[]> {
        return await invoke('get_lesson_tests', { id: lessonId });
    }
}