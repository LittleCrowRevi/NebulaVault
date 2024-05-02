using System;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;

[ExecuteInEditMode]
public class FlexibleUi : MonoBehaviour
{
    public FlexibleUiData skinData;

    protected virtual void ModifySkinUi()
    {
        
    }

    public void Awake()
    {
        ModifySkinUi();
    }

    public virtual void Update()
    {
        if ( Application.isEditor )
        {
            ModifySkinUi();
        }
    }
}
