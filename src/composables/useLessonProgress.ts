import { ref, computed, ComputedRef } from 'vue';
import { difficultyRequirements } from '../types';
import { User } from '../types/bindings';

interface LessonProgressParams {
    user: User | null;
    accuracy: ComputedRef<number>;
    wpm: ComputedRef<number>;
}

export default function useLessonProgress({ user, accuracy, wpm }: LessonProgressParams) {
    const lessonCompleted = ref(false);
    const grade = ref('');

    const userRequirements = computed(() => {
        if (!user?.lessonDifficulty) return { accuracy: 0, wpm: 0 };
        return difficultyRequirements[user.lessonDifficulty.toLowerCase()] || { accuracy: 0, wpm: 0 };
    });

    const passedLesson = computed(() => {
        if (!user?.lessonDifficulty) return false;
        return accuracy.value >= userRequirements.value.accuracy && wpm.value >= userRequirements.value.wpm;
    });

    const calculateGrade = () => {
        const requirements = userRequirements.value;

        if (accuracy.value >= requirements.accuracy + 10 && wpm.value >= requirements.wpm + 10) return 'S';
        if (accuracy.value >= requirements.accuracy + 5 && wpm.value >= requirements.wpm + 5) return 'A';
        if (accuracy.value >= requirements.accuracy && wpm.value >= requirements.wpm) return 'B';
        if (accuracy.value >= requirements.accuracy - 10 && wpm.value >= requirements.wpm - 5) return 'C';
        return 'D';
    };

    const setGrade = () => {
        grade.value = calculateGrade();
    };

    const markCompleted = () => {
        lessonCompleted.value = true;
    };

    const reset = () => {
        lessonCompleted.value = false;
        grade.value = '';
    };

    return {
        lessonCompleted: computed(() => lessonCompleted.value),
        grade: computed(() => grade.value),
        userRequirements,
        passedLesson,
        calculateGrade,
        setGrade,
        markCompleted,
        reset
    };
}