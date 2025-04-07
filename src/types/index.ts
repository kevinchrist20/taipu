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