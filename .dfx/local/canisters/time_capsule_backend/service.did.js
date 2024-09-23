export const idlFactory = ({ IDL }) => {
  const TimeCapsule = IDL.Record({
    'unlock_time' : IDL.Nat64,
    'owner' : IDL.Text,
    'message' : IDL.Text,
  });
  return IDL.Service({
    'create_time_capsule' : IDL.Func(
        [IDL.Text, IDL.Text, IDL.Nat64],
        [IDL.Variant({ 'Ok' : IDL.Nat64, 'Err' : IDL.Text })],
        [],
      ),
    'delete_time_capsule' : IDL.Func(
        [IDL.Nat64],
        [IDL.Variant({ 'Ok' : IDL.Text, 'Err' : IDL.Text })],
        [],
      ),
    'get_time_capsule' : IDL.Func(
        [IDL.Nat64],
        [IDL.Variant({ 'Ok' : IDL.Text, 'Err' : IDL.Text })],
        ['query'],
      ),
    'greet' : IDL.Func([IDL.Text], [IDL.Text], ['query']),
    'list_available_capsules' : IDL.Func(
        [],
        [IDL.Vec(IDL.Tuple(IDL.Nat64, IDL.Text))],
        ['query'],
      ),
    'list_capsules_by_owner' : IDL.Func(
        [],
        [IDL.Vec(IDL.Tuple(IDL.Nat64, TimeCapsule))],
        ['query'],
      ),
    'update_time_capsule' : IDL.Func(
        [IDL.Nat64, IDL.Opt(IDL.Text), IDL.Opt(IDL.Nat64)],
        [IDL.Variant({ 'Ok' : IDL.Text, 'Err' : IDL.Text })],
        [],
      ),
  });
};
export const init = ({ IDL }) => { return []; };
