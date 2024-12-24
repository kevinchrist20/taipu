import { Lesson } from '../types/bindings';
import { invoke } from '../types/tauri_commands';

export default {
    async getLessons(difficulty: string): Promise<Lesson[]> {
        const lessons = await invoke('get_lessons_by_difficulty', { 
            difficulty: difficulty.toUpperCase() 
        });
        return lessons;
    },
}