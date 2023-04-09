import {
	_inferProcedureHandlerInput,
	createClient,
	FetchTransport,
	inferMutationInput,
	inferMutationResult,
	inferProcedures,
	ProceduresLike,
	RSPCError,
	WebsocketTransport,
} from "@rspc/client";
import { BaseOptions, createReactQueryHooks } from "@rspc/react";
import {
	QueryClient,
	UseMutateFunction,
	UseMutationOptions,
	UseMutationResult,
} from "@tanstack/react-query";
import { TauriTransport } from "@rspc/tauri";
import { Procedures } from "./types/rspc/bindings";

export const client = createClient<Procedures>({
	transport:
		//   typeof window === "undefined"
		// 	? // WebsocketTransport can not be used Server Side, so we provide FetchTransport instead.
		// 	  // If you do not plan on using Subscriptions you can use FetchTransport on Client Side as well.
		// 	  new FetchTransport("http://localhost:4000/rspc")
		// 	: new WebsocketTransport("ws://localhost:4000/rspc/ws"),
		new TauriTransport(),
});

export const queryClient = new QueryClient({
	defaultOptions: {
		queries: {
			retry: false, // If you want to retry when requests fail, remove this.
		},
	},
});

type UseMutationStateResult<
	K_3 extends inferProcedures<Procedures>["mutations"]["key"] & string,
	TContext_1 = unknown
> = UseMutationResult<
	inferMutationResult<inferProcedures<Procedures>, K_3>,
	RSPCError,
	inferMutationInput<inferProcedures<Procedures>, K_3> extends never
		? undefined
		: inferMutationInput<inferProcedures<Procedures>, K_3>,
	TContext_1
>;
export const useMutationState = <
	TProceduresLike extends Procedures,
	K_3 extends inferProcedures<TProceduresLike>["mutations"]["key"] & string,
	K_4 extends inferProcedures<TProceduresLike>["subscriptions"]["key"] & string,
	TContext_1 = unknown
>(
	key: K_3 | [K_3],
	subscription?: K_3 & K_4, // [key: K_3, ...input: _inferProcedureHandlerInput<inferProcedures<TProceduresLike>, "subscriptions", K_4>],
	opts?: UseMutationOptions<
		inferMutationResult<inferProcedures<TProceduresLike>, K_3>,
		RSPCError,
		inferMutationInput<inferProcedures<TProceduresLike>, K_3> extends never
			? undefined
			: inferMutationInput<inferProcedures<TProceduresLike>, K_3>,
		TContext_1
	> &
		BaseOptions<inferProcedures<TProceduresLike>>
	// | undefined
): [
	UseMutationStateResult<K_3, TContext_1>["data"],
	UseMutationStateResult<K_3, TContext_1>["mutate"]
] => {
	const { mutate, data, isLoading, error } = useMutation(key, opts as any);
	if (subscription) {
		useSubscription([subscription] as any, {
			onData: (v) => {
				console.log("^_^ Log \n file: App.tsx:132 \n v:", v);
				mutate(undefined);

				// console.log("^_^ Log \n file: App.tsx:134 \n v:", v);

				// setPings((currentPings) => currentPings + 1);
			},
		});
	}
	//@ts-expect-error
	return [data, mutate];
};
// type t1=
// 	UseMutationOptions<inferMutationResult<Procedures, K_3>, RSPCError, inferMutationInput<Procedures, K_3> extends never ? undefined : inferMutationInput<...>, TContext_1 > & BaseOptions <...>;
// type t2 = (UseMutationOptions<inferMutationResult<Procedures, K_3>, RSPCError, inferMutationInput<Procedures, K_3> extends never ? undefined : inferMutationInput<...>, TContext_1 > & BaseOptions <...>) | undefined;
export const {
	useContext,
	useMutation,
	useQuery,
	useSubscription,
	Provider: RSPCProvider,
} = createReactQueryHooks<Procedures>();

// export const tauriClient = createClient<Procedures>({
// 	transport: new TauriTransport(),
// })
