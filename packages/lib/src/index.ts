export default {
  setServerSideI18NesteState: <Ctx, F extends (ctx: Ctx) => Promise<any>>(
    f: F
  ) => {
    console.log("pluginen funker som faen den!!");

    return (ctx: Ctx) => f(ctx);
  },
};
