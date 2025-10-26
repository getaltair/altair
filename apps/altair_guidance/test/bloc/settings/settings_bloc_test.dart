import 'package:altair_guidance/bloc/settings/settings_bloc.dart';
import 'package:altair_guidance/bloc/settings/settings_event.dart';
import 'package:altair_guidance/bloc/settings/settings_state.dart';
import 'package:altair_guidance/models/ai_settings.dart';
import 'package:altair_guidance/repositories/ai_settings_repository.dart';
import 'package:bloc_test/bloc_test.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:mocktail/mocktail.dart';

class MockAISettingsRepository extends Mock implements AISettingsRepository {}

class FakeAISettings extends Fake implements AISettings {}

void main() {
  late MockAISettingsRepository mockRepository;

  setUpAll(() {
    registerFallbackValue(FakeAISettings());
  });

  setUp(() {
    mockRepository = MockAISettingsRepository();
  });

  group('SettingsBloc', () {
    test('initial state is SettingsInitial', () {
      final bloc = SettingsBloc(aiSettingsRepository: mockRepository);
      expect(bloc.state, const SettingsInitial());
      bloc.close();
    });

    group('SettingsLoadRequested', () {
      blocTest<SettingsBloc, SettingsState>(
        'emits [SettingsLoading, SettingsLoaded] when load succeeds',
        build: () {
          const testSettings = AISettings(
            enabled: true,
            provider: AIProviderType.openai,
            openaiApiKey: 'test-key',
          );
          when(() => mockRepository.load())
              .thenAnswer((_) async => testSettings);
          return SettingsBloc(aiSettingsRepository: mockRepository);
        },
        act: (bloc) => bloc.add(const SettingsLoadRequested()),
        expect: () => const [
          SettingsLoading(),
          SettingsLoaded(
            AISettings(
              enabled: true,
              provider: AIProviderType.openai,
              openaiApiKey: 'test-key',
            ),
          ),
        ],
        verify: (_) {
          verify(() => mockRepository.load()).called(1);
        },
      );

      blocTest<SettingsBloc, SettingsState>(
        'emits [SettingsLoading, SettingsFailure] when load fails',
        build: () {
          when(() => mockRepository.load())
              .thenThrow(Exception('Load failed'));
          return SettingsBloc(aiSettingsRepository: mockRepository);
        },
        act: (bloc) => bloc.add(const SettingsLoadRequested()),
        expect: () => [
          const SettingsLoading(),
          isA<SettingsFailure>().having(
            (state) => state.message,
            'message',
            contains('Failed to load settings'),
          ),
        ],
      );

      blocTest<SettingsBloc, SettingsState>(
        'loads default settings when repository returns defaults',
        build: () {
          when(() => mockRepository.load())
              .thenAnswer((_) async => const AISettings());
          return SettingsBloc(aiSettingsRepository: mockRepository);
        },
        act: (bloc) => bloc.add(const SettingsLoadRequested()),
        expect: () => const [
          SettingsLoading(),
          SettingsLoaded(AISettings()),
        ],
      );
    });

    group('SettingsAIUpdated', () {
      const testSettings = AISettings(
        enabled: true,
        provider: AIProviderType.anthropic,
        anthropicApiKey: 'test-key',
      );

      blocTest<SettingsBloc, SettingsState>(
        'emits [SettingsLoaded] with new settings',
        build: () {
          when(() => mockRepository.save(any())).thenAnswer((_) async {});
          return SettingsBloc(aiSettingsRepository: mockRepository);
        },
        act: (bloc) => bloc.add(const SettingsAIUpdated(testSettings)),
        expect: () => const [
          SettingsLoaded(testSettings),
          SettingsSaving(testSettings),
          SettingsSaved(testSettings),
          SettingsLoaded(testSettings),
        ],
        verify: (_) {
          verify(() => mockRepository.save(testSettings)).called(1);
        },
      );

      blocTest<SettingsBloc, SettingsState>(
        'auto-saves after update',
        build: () {
          when(() => mockRepository.save(any())).thenAnswer((_) async {});
          return SettingsBloc(aiSettingsRepository: mockRepository);
        },
        act: (bloc) => bloc.add(const SettingsAIUpdated(testSettings)),
        verify: (_) {
          verify(() => mockRepository.save(testSettings)).called(1);
        },
      );
    });

    group('SettingsAIToggled', () {
      blocTest<SettingsBloc, SettingsState>(
        'does nothing when settings not loaded',
        build: () => SettingsBloc(aiSettingsRepository: mockRepository),
        act: (bloc) => bloc.add(const SettingsAIToggled(true)),
        expect: () => [],
        verify: (_) {
          verifyNever(() => mockRepository.save(any()));
        },
      );

      blocTest<SettingsBloc, SettingsState>(
        'toggles enabled to true and auto-saves',
        build: () {
          const initialSettings = AISettings(enabled: false);
          when(() => mockRepository.load())
              .thenAnswer((_) async => initialSettings);
          when(() => mockRepository.save(any())).thenAnswer((_) async {});
          return SettingsBloc(aiSettingsRepository: mockRepository);
        },
        act: (bloc) {
          bloc.add(const SettingsLoadRequested());
          return bloc.stream.firstWhere((state) => state is SettingsLoaded).then(
                (_) => bloc.add(const SettingsAIToggled(true)),
              );
        },
        skip: 2, // Skip initial load states
        expect: () => const [
          SettingsLoaded(AISettings(enabled: true)),
          SettingsSaving(AISettings(enabled: true)),
          SettingsSaved(AISettings(enabled: true)),
          SettingsLoaded(AISettings(enabled: true)),
        ],
      );

      blocTest<SettingsBloc, SettingsState>(
        'toggles enabled to false and auto-saves',
        build: () {
          const initialSettings = AISettings(enabled: true);
          when(() => mockRepository.load())
              .thenAnswer((_) async => initialSettings);
          when(() => mockRepository.save(any())).thenAnswer((_) async {});
          return SettingsBloc(aiSettingsRepository: mockRepository);
        },
        act: (bloc) {
          bloc.add(const SettingsLoadRequested());
          return bloc.stream.firstWhere((state) => state is SettingsLoaded).then(
                (_) => bloc.add(const SettingsAIToggled(false)),
              );
        },
        skip: 2, // Skip initial load states
        expect: () => const [
          SettingsLoaded(AISettings(enabled: false)),
          SettingsSaving(AISettings(enabled: false)),
          SettingsSaved(AISettings(enabled: false)),
          SettingsLoaded(AISettings(enabled: false)),
        ],
      );

      blocTest<SettingsBloc, SettingsState>(
        'preserves other settings when toggling',
        build: () {
          const initialSettings = AISettings(
            enabled: false,
            provider: AIProviderType.openai,
            openaiModel: 'gpt-4',
          );
          when(() => mockRepository.load())
              .thenAnswer((_) async => initialSettings);
          when(() => mockRepository.save(any())).thenAnswer((_) async {});
          return SettingsBloc(aiSettingsRepository: mockRepository);
        },
        act: (bloc) {
          bloc.add(const SettingsLoadRequested());
          return bloc.stream.firstWhere((state) => state is SettingsLoaded).then(
                (_) => bloc.add(const SettingsAIToggled(true)),
              );
        },
        skip: 2, // Skip initial load states
        expect: () => const [
          SettingsLoaded(
            AISettings(
              enabled: true,
              provider: AIProviderType.openai,
              openaiModel: 'gpt-4',
            ),
          ),
          SettingsSaving(
            AISettings(
              enabled: true,
              provider: AIProviderType.openai,
              openaiModel: 'gpt-4',
            ),
          ),
          SettingsSaved(
            AISettings(
              enabled: true,
              provider: AIProviderType.openai,
              openaiModel: 'gpt-4',
            ),
          ),
          SettingsLoaded(
            AISettings(
              enabled: true,
              provider: AIProviderType.openai,
              openaiModel: 'gpt-4',
            ),
          ),
        ],
      );
    });

    group('SettingsSaveRequested', () {
      blocTest<SettingsBloc, SettingsState>(
        'does nothing when no settings loaded',
        build: () => SettingsBloc(aiSettingsRepository: mockRepository),
        act: (bloc) => bloc.add(const SettingsSaveRequested()),
        expect: () => [],
      );

      blocTest<SettingsBloc, SettingsState>(
        'emits [SettingsSaving, SettingsSaved, SettingsLoaded] when save succeeds',
        build: () {
          const testSettings = AISettings(enabled: true);
          when(() => mockRepository.load())
              .thenAnswer((_) async => testSettings);
          when(() => mockRepository.save(any())).thenAnswer((_) async {});
          return SettingsBloc(aiSettingsRepository: mockRepository);
        },
        act: (bloc) {
          bloc.add(const SettingsLoadRequested());
          return bloc.stream.firstWhere((state) => state is SettingsLoaded).then(
                (_) => bloc.add(const SettingsSaveRequested()),
              );
        },
        skip: 2, // Skip initial load states
        expect: () => const [
          SettingsSaving(AISettings(enabled: true)),
          SettingsSaved(AISettings(enabled: true)),
          SettingsLoaded(AISettings(enabled: true)),
        ],
        verify: (_) {
          verify(() => mockRepository.save(const AISettings(enabled: true)))
              .called(1);
        },
      );

      blocTest<SettingsBloc, SettingsState>(
        'emits [SettingsSaving, SettingsFailure, SettingsLoaded] when save fails',
        build: () {
          const testSettings = AISettings(enabled: true);
          when(() => mockRepository.load())
              .thenAnswer((_) async => testSettings);
          when(() => mockRepository.save(any()))
              .thenThrow(Exception('Save failed'));
          return SettingsBloc(aiSettingsRepository: mockRepository);
        },
        act: (bloc) {
          bloc.add(const SettingsLoadRequested());
          return bloc.stream.firstWhere((state) => state is SettingsLoaded).then(
                (_) => bloc.add(const SettingsSaveRequested()),
              );
        },
        skip: 2, // Skip initial load states
        expect: () => [
          const SettingsSaving(AISettings(enabled: true)),
          isA<SettingsFailure>()
              .having(
                (state) => state.message,
                'message',
                contains('Failed to save settings'),
              )
              .having(
                (state) => state.aiSettings,
                'aiSettings',
                const AISettings(enabled: true),
              ),
          const SettingsLoaded(AISettings(enabled: true)),
        ],
      );
    });

    group('SettingsClearRequested', () {
      blocTest<SettingsBloc, SettingsState>(
        'emits [SettingsLoading, SettingsLoaded] with default settings when clear succeeds',
        build: () {
          when(() => mockRepository.clear()).thenAnswer((_) async {});
          return SettingsBloc(aiSettingsRepository: mockRepository);
        },
        act: (bloc) => bloc.add(const SettingsClearRequested()),
        expect: () => const [
          SettingsLoading(),
          SettingsLoaded(AISettings()),
        ],
        verify: (_) {
          verify(() => mockRepository.clear()).called(1);
        },
      );

      blocTest<SettingsBloc, SettingsState>(
        'emits [SettingsLoading, SettingsFailure] when clear fails',
        build: () {
          when(() => mockRepository.clear())
              .thenThrow(Exception('Clear failed'));
          return SettingsBloc(aiSettingsRepository: mockRepository);
        },
        act: (bloc) => bloc.add(const SettingsClearRequested()),
        expect: () => [
          const SettingsLoading(),
          isA<SettingsFailure>().having(
            (state) => state.message,
            'message',
            contains('Failed to clear settings'),
          ),
        ],
      );
    });

    group('Integration scenarios', () {
      blocTest<SettingsBloc, SettingsState>(
        'can load, update, and save settings',
        build: () {
          const initialSettings = AISettings(enabled: false);
          when(() => mockRepository.load())
              .thenAnswer((_) async => initialSettings);
          when(() => mockRepository.save(any())).thenAnswer((_) async {});
          return SettingsBloc(aiSettingsRepository: mockRepository);
        },
        act: (bloc) async {
          bloc.add(const SettingsLoadRequested());
          await bloc.stream.firstWhere((state) => state is SettingsLoaded);
          bloc.add(
            const SettingsAIUpdated(
              AISettings(
                enabled: true,
                provider: AIProviderType.openai,
                openaiApiKey: 'test-key',
              ),
            ),
          );
        },
        expect: () => const [
          SettingsLoading(),
          SettingsLoaded(AISettings(enabled: false)),
          SettingsLoaded(
            AISettings(
              enabled: true,
              provider: AIProviderType.openai,
              openaiApiKey: 'test-key',
            ),
          ),
          SettingsSaving(
            AISettings(
              enabled: true,
              provider: AIProviderType.openai,
              openaiApiKey: 'test-key',
            ),
          ),
          SettingsSaved(
            AISettings(
              enabled: true,
              provider: AIProviderType.openai,
              openaiApiKey: 'test-key',
            ),
          ),
          SettingsLoaded(
            AISettings(
              enabled: true,
              provider: AIProviderType.openai,
              openaiApiKey: 'test-key',
            ),
          ),
        ],
      );

      blocTest<SettingsBloc, SettingsState>(
        'can load and clear settings',
        build: () {
          const initialSettings = AISettings(
            enabled: true,
            provider: AIProviderType.openai,
          );
          when(() => mockRepository.load())
              .thenAnswer((_) async => initialSettings);
          when(() => mockRepository.clear()).thenAnswer((_) async {});
          return SettingsBloc(aiSettingsRepository: mockRepository);
        },
        act: (bloc) async {
          bloc.add(const SettingsLoadRequested());
          await bloc.stream.firstWhere((state) => state is SettingsLoaded);
          bloc.add(const SettingsClearRequested());
        },
        expect: () => const [
          SettingsLoading(),
          SettingsLoaded(
            AISettings(enabled: true, provider: AIProviderType.openai),
          ),
          SettingsLoading(),
          SettingsLoaded(AISettings()),
        ],
      );

      test('multiple toggles work correctly', () async {
        const initialSettings = AISettings(enabled: false);
        when(() => mockRepository.load())
            .thenAnswer((_) async => initialSettings);
        when(() => mockRepository.save(any())).thenAnswer((_) async {});

        final bloc = SettingsBloc(aiSettingsRepository: mockRepository);

        // Load initial settings
        bloc.add(const SettingsLoadRequested());
        await expectLater(
          bloc.stream,
          emitsInOrder([
            const SettingsLoading(),
            const SettingsLoaded(AISettings(enabled: false)),
          ]),
        );

        // First toggle to true
        bloc.add(const SettingsAIToggled(true));
        await expectLater(
          bloc.stream,
          emitsInOrder([
            const SettingsLoaded(AISettings(enabled: true)),
            const SettingsSaving(AISettings(enabled: true)),
            const SettingsSaved(AISettings(enabled: true)),
            const SettingsLoaded(AISettings(enabled: true)),
          ]),
        );

        // Second toggle to false
        bloc.add(const SettingsAIToggled(false));
        await expectLater(
          bloc.stream,
          emitsInOrder([
            const SettingsLoaded(AISettings(enabled: false)),
            const SettingsSaving(AISettings(enabled: false)),
            const SettingsSaved(AISettings(enabled: false)),
            const SettingsLoaded(AISettings(enabled: false)),
          ]),
        );

        await bloc.close();

        // Verify save was called twice
        verify(() => mockRepository.save(any())).called(2);
      });
    });
  });
}
