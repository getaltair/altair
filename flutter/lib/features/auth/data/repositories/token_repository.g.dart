// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'token_repository.dart';

// **************************************************************************
// RiverpodGenerator
// **************************************************************************

// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint, type=warning
/// Provides a singleton instance of [TokenRepository]

@ProviderFor(tokenRepository)
const tokenRepositoryProvider = TokenRepositoryProvider._();

/// Provides a singleton instance of [TokenRepository]

final class TokenRepositoryProvider
    extends
        $FunctionalProvider<TokenRepository, TokenRepository, TokenRepository>
    with $Provider<TokenRepository> {
  /// Provides a singleton instance of [TokenRepository]
  const TokenRepositoryProvider._()
    : super(
        from: null,
        argument: null,
        retry: null,
        name: r'tokenRepositoryProvider',
        isAutoDispose: true,
        dependencies: null,
        $allTransitiveDependencies: null,
      );

  @override
  String debugGetCreateSourceHash() => _$tokenRepositoryHash();

  @$internal
  @override
  $ProviderElement<TokenRepository> $createElement($ProviderPointer pointer) =>
      $ProviderElement(pointer);

  @override
  TokenRepository create(Ref ref) {
    return tokenRepository(ref);
  }

  /// {@macro riverpod.override_with_value}
  Override overrideWithValue(TokenRepository value) {
    return $ProviderOverride(
      origin: this,
      providerOverride: $SyncValueProvider<TokenRepository>(value),
    );
  }
}

String _$tokenRepositoryHash() => r'77095c5c12c3934e4d51b7da14658a1452633b5e';
