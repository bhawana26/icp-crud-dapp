export const idlFactory = ({ IDL }) => {
  return IDL.Service({
    'create_item' : IDL.Func([IDL.Nat32, IDL.Text], [], []),
    'delete_item' : IDL.Func([IDL.Nat32], [], []),
    'read_item' : IDL.Func([IDL.Nat32], [IDL.Opt(IDL.Text)], ['query']),
    'update_item' : IDL.Func([IDL.Nat32, IDL.Text], [], []),
  });
};
export const init = ({ IDL }) => { return []; };
