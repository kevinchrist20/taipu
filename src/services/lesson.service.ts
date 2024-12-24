import { SessionStore } from '../storage';
import { Lesson } from '../types/bindings';
import { invoke } from '../types/tauri_commands';

export default {
    async getLessons(): Promise<Lesson[]> {
        const user = SessionStore.user;

        const lessons = await invoke(
            'get_lessons_by_difficulty',
            { difficulty: user?.difficulty }
        );
        
        return lessons;
    },
}