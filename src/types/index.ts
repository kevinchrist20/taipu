export * from './key_types';
export * from './global';
export * from './tauri_commands';
export * from './bindings';

export interface DifficultyRequirement {
  accuracy: number;
  wpm: number;
}

export const difficultyRequirements: Record<string, DifficultyRequirement> = {
  beginner: { accuracy: 80, wpm: 15 },
  intermediate: { accuracy: 85, wpm: 30 },
  advanced: { accuracy: 90, wpm: 40 }
};

export type CategoryStatus = 'locked' | 'unlocked' | 'completed';
export type LessonStatus = 'not_started' | 'in_progress' | 'completed';
export  type TestStatus = 'Passed' | 'Ready' | 'Locked';