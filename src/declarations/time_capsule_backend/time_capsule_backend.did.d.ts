import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export interface TimeCapsule {
  'unlock_time' : bigint,
  'owner' : string,
  'message' : string,
}
export interface _SERVICE {
  'create_time_capsule' : ActorMethod<
    [string, string, bigint],
    { 'Ok' : bigint } |
      { 'Err' : string }
  >,
  'delete_time_capsule' : ActorMethod<
    [bigint],
    { 'Ok' : string } |
      { 'Err' : string }
  >,
  'get_time_capsule' : ActorMethod<
    [bigint],
    { 'Ok' : string } |
      { 'Err' : string }
  >,
  'greet' : ActorMethod<[string], string>,
  'list_available_capsules' : ActorMethod<[], Array<[bigint, string]>>,
  'list_capsules_by_owner' : ActorMethod<[], Array<[bigint, TimeCapsule]>>,
  'update_time_capsule' : ActorMethod<
    [bigint, [] | [string], [] | [bigint]],
    { 'Ok' : string } |
      { 'Err' : string }
  >,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
